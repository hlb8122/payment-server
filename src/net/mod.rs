pub mod errors;
pub mod jsonrpc_client;

use std::str;

use actix_web::{
    http::header::{HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, LOCATION, PRAGMA},
    web, HttpRequest, HttpResponse,
};
use bitcoin::{util::psbt::serialize::Deserialize, Transaction};
use bitcoincash_addr::HashType;
use bytes::BytesMut;

use futures::{
    future::{err, Either, Future},
    stream::Stream,
};
use prost::Message;
use url::Url;
use uuid::Uuid;

use crate::{
    bitcoin::*,
    crypto::{AddressCodec, CashAddrCodec},
    models::*,
    sql::postgresql::*,
    ConnPool, SETTINGS,
};

use errors::*;

pub const VALID_DURATION: u64 = 30;

// Payment handler
pub fn payment_handler(
    req: HttpRequest,
    payment_id: web::Path<String>,
    payload: web::Payload,
    data: web::Data<(BitcoinClient, ConnPool)>,
) -> Box<dyn Future<Item = HttpResponse, Error = ServerError>> {
    // Check headers
    let headers = req.headers();
    if headers.get(CONTENT_TYPE)
        != Some(&HeaderValue::from_str("application/bitcoincash-payment").unwrap())
    {
        return Box::new(err(PaymentError::Accept.into()));
    }
    if headers.get(ACCEPT)
        != Some(&HeaderValue::from_str("application/bitcoincash-paymentack").unwrap())
    {
        return Box::new(err(PaymentError::Content.into()));
    }

    // Read and parse payment proto
    let body_raw =
        payload
            .map_err(|_| PaymentError::Payload)
            .fold(BytesMut::new(), move |mut body, chunk| {
                body.extend_from_slice(&chunk);
                Ok::<_, PaymentError>(body)
            });
    let payment = body_raw
        .and_then(|payment_raw| Payment::decode(payment_raw).map_err(|_| PaymentError::Decode));

    let payment_ack = payment
        .and_then(move |payment| {
            // Parse tx
            let tx_raw = match payment.transactions.get(0) {
                Some(some) => some,
                None => return Either::A(err(PaymentError::NoTx)),
            }; // Assume first tx
            let tx = match Transaction::deserialize(tx_raw) {
                Ok(ok) => ok,
                Err(e) => return Either::A(err(PaymentError::from(e))),
            };

            // Check outputs
            // let wallet_data = &data.1;
            // if !wallet_data.check_outputs(tx) {
            //     return Either::A(err(PaymentError::InvalidOutputs));
            // }

            // Send tx
            let bitcoin_client = &data.0;
            Either::B(
                bitcoin_client
                    .send_tx(tx_raw)
                    .and_then(|_txid| {
                        // Create payment ack
                        let memo = Some("Thanks for your custom!".to_string());
                        Ok(PaymentAck { payment, memo })
                    })
                    .map_err(|_| PaymentError::InvalidTx),
            )
        })
        .map_err(ServerError::Payment);

    let response = payment_ack.and_then(|ack| {
        // Encode payment ack
        let mut raw_ack = Vec::with_capacity(ack.encoded_len());
        ack.encode(&mut raw_ack).unwrap();

        // Get merchant data
        let merchant_data = ack
            .payment
            .merchant_data
            .ok_or(PaymentError::NoMerchantDat)?;

        // Sign token
        let token = "TODO: Get token from mysql then sign it".to_string();

        // Generate payment redirect
        let mut redirect_url = Url::parse(
            str::from_utf8(&merchant_data).map_err(|_| PaymentError::InvalidMerchantDat)?,
        )
        .map_err(|_| PaymentError::InvalidMerchantDat)?;
        redirect_url.set_query(Some(&format!("code={}", token)));

        // Generate response
        Ok(HttpResponse::Found()
            .header(LOCATION, redirect_url.into_string())
            .header(AUTHORIZATION, format!("POP {}", token))
            .header(PRAGMA, "no-cache")
            .body(raw_ack))
    });

    Box::new(response)
}

pub fn generate_invoice(
    req: HttpRequest,
    payload: web::Payload,
    data: web::Data<(BitcoinClient, ConnPool)>,
) -> Box<dyn Future<Item = HttpResponse, Error = ServerError>> {
    let mut client = data.0.to_owned();
    let conn = data.1.to_owned();

    // Decode metadata
    let body_raw = payload.map_err(|_| ServerError::InvoiceRequestDecode).fold(
        BytesMut::new(),
        move |mut body, chunk| {
            body.extend_from_slice(&chunk);
            Ok::<_, ServerError>(body)
        },
    );
    let fut_invoice_request = body_raw.and_then(|metadata_raw| {
        InvoiceRequest::decode(metadata_raw).map_err(|_| ServerError::InvoiceRequestDecode)
    });

    // Get new addr and add to wallet
    let new_addr = client.get_new_addr().then(move |addr_opt| match addr_opt {
        Ok(str_addr) => {
            let addr = CashAddrCodec::decode(&str_addr).map_err(ServerError::Address)?;
            let network: Network = addr.network.clone().into();
            if network != SETTINGS.network || addr.hash_type != HashType::Key {
                return Err(ServerError::Payment(PaymentError::MismatchedNetwork))?; // TODO: Finer grained error here
            }
            Ok((addr.into_body(), str_addr))
        }
        Err(_e) => Err(ServerError::Payment(PaymentError::AddrFetchFailed)),
    });

    let generate = fut_invoice_request.join(new_addr).and_then(
        move |(invoice_request, (raw_addr, str_addr))| {
            // Generate outputs
            let outputs =
                generate_outputs(&raw_addr, invoice_request.amount, &invoice_request.tx_data);

            // Generate payment details
            let id = Uuid::new_v4();
            let expires = match invoice_request.expires {
                0 => None,
                some => Some(some),
            };
            let callback_url = match invoice_request.callback_url.as_str() {
                "" => None,
                value => Some(value),
            };
            let merchant_data = if invoice_request.merchant_data.is_empty() {
                None
            } else {
                Some(invoice_request.merchant_data)
            };
            let req_memo = match invoice_request.req_memo.as_str() {
                "" => None,
                value => Some(value),
            };
            let payment_details = PaymentDetails {
                network: Some(SETTINGS.network.to_string()),
                payment_url: Some(format!("{}{}:", SETTINGS.payment_url, &id.to_string())),
                memo: req_memo.map(|value| value.to_string()),
                expires,
                time: invoice_request.time,
                merchant_data,
                outputs,
            };

            // Add row to SQL table
            let token_data = if invoice_request.token.is_empty() {
                None
            } else {
                Some(&invoice_request.token[..])
            };
            let ack_memo = match invoice_request.ack_memo.as_str() {
                "" => None,
                value => Some(value),
            };
            let tx_data = if invoice_request.tx_data.is_empty() {
                None
            } else {
                Some(&invoice_request.tx_data[..])
            };
            let connection = conn.get().unwrap();
            let fut_add_payment = add_payment(
                &payment_details,
                &id,
                &str_addr,
                invoice_request.amount as i64,
                req_memo,
                callback_url,
                ack_memo,
                token_data,
                tx_data,
                &connection,
            );
            let mut serialized_payment_details = Vec::with_capacity(payment_details.encoded_len());
            payment_details
                .encode(&mut serialized_payment_details)
                .unwrap();
            actix_web::web::block(|| fut_add_payment)
                .map_err(|err| match err {
                    actix_threadpool::BlockingError::Error(e) => e.into(),
                    _ => unreachable!(),
                })
                .map(move |_| (id.to_string(), serialized_payment_details))
        },
    );

    let response = generate.and_then(|(payment_id, serialized_payment_details)| {
        // Generate payment invoice
        // TODO: Sign here
        let pki_type = Some("none".to_string());
        let payment_request = PaymentRequest {
            pki_type,
            pki_data: None,
            payment_details_version: Some(1),
            serialized_payment_details,
            signature: None,
        };
        let invoice_response = InvoiceResponse {
            payment_id,
            payment_request: Some(payment_request),
        };
        let mut raw_invoice_response = Vec::with_capacity(invoice_response.encoded_len());
        invoice_response.encode(&mut raw_invoice_response).unwrap();

        Ok(HttpResponse::PaymentRequired()
            .content_type("application/bitcoincash-paymentrequest")
            .header("Content-Transfer-Encoding", "binary")
            .body(raw_invoice_response))
    });

    // Respond
    Box::new(response)
}
