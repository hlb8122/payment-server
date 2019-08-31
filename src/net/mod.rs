pub mod errors;
pub mod jsonrpc_client;

use std::{
    str,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use actix_web::{
    http::{
        header::{HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, LOCATION, PRAGMA},
    },
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

use crate::{
    bitcoin::*,
    crypto::{AddressCodec, CashAddrCodec},
    models::*,
    SETTINGS,
};

use errors::*;

pub const VALID_DURATION: u64 = 30;

// Payment handler
pub fn payment_handler(
    req: HttpRequest,
    payload: web::Payload,
    data: web::Data<(BitcoinClient, WalletState)>,
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
            let wallet_data = &data.1;
            if !wallet_data.check_outputs(tx) {
                return Either::A(err(PaymentError::InvalidOutputs));
            }

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
    data: web::Data<(BitcoinClient, WalletState)>,
) -> Box<dyn Future<Item = HttpResponse, Error = ServerError>> {
    let mut client = data.0.to_owned();

    // Decode metadata
    let body_raw = payload.map_err(|_| ServerError::InvoiceParamsDecode).fold(
        BytesMut::new(),
        move |mut body, chunk| {
            body.extend_from_slice(&chunk);
            Ok::<_, ServerError>(body)
        },
    );
    let invoice_params_fut = body_raw.and_then(|metadata_raw| {
        InvoiceParams::decode(metadata_raw).map_err(|_| ServerError::InvoiceParamsDecode)
    });

    // Valid interval
    let current_time = SystemTime::now();
    let expiry_time = current_time + Duration::from_secs(VALID_DURATION);

    // Get new addr and add to wallet
    let new_addr = client.get_new_addr().then(move |addr_opt| match addr_opt {
        Ok(addr_str) => {
            let addr = CashAddrCodec::decode(&addr_str).map_err(ServerError::Address)?;
            let network: Network = addr.network.clone().into();
            if network != SETTINGS.network || addr.hash_type != HashType::Key {
                return Err(ServerError::Payment(PaymentError::MismatchedNetwork))?; // TODO: Finer grained error here
            }
            let addr_raw = addr.into_body();
            data.1.add(addr_raw.clone());
            Ok(addr_raw)
        }
        Err(_e) => Err(ServerError::Payment(PaymentError::AddrFetchFailed)),
    });

    let response = invoice_params_fut
        .join(new_addr)
        .and_then(move |(invoice_params, addr_raw)| {
            // Generate outputs
            let outputs = generate_outputs(addr_raw, invoice_params.tx_data);

            // Collect payment details
            let payment_details = PaymentDetails {
                network: Some(SETTINGS.network.to_string()),
                time: current_time.duration_since(UNIX_EPOCH).unwrap().as_secs(),
                expires: Some(expiry_time.duration_since(UNIX_EPOCH).unwrap().as_secs()),
                memo: None,
                merchant_data: Some(invoice_params.merchant_data),
                outputs,
                payment_url: Some(SETTINGS.payment_url.to_string()),
            };
            let mut serialized_payment_details = Vec::with_capacity(payment_details.encoded_len());
            payment_details
                .encode(&mut serialized_payment_details)
                .unwrap();

            // Generate payment invoice
            let pki_type = Some("none".to_string());
            let payment_invoice = PaymentRequest {
                pki_type,
                pki_data: None,
                payment_details_version: Some(1),
                serialized_payment_details,
                signature: None,
            };
            let mut payment_invoice_raw = Vec::with_capacity(payment_invoice.encoded_len());
            payment_invoice.encode(&mut payment_invoice_raw).unwrap();

            Ok(HttpResponse::PaymentRequired()
                .content_type("application/bitcoincash-paymentrequest")
                .header("Content-Transfer-Encoding", "binary")
                .body(payment_invoice_raw))
        });

    // Respond
    Box::new(response)
}

/*
Payment middleware
*/
pub struct CheckPayment(BitcoinClient, WalletState);

impl CheckPayment {
    pub fn new(client: BitcoinClient, wallet_state: WalletState) -> Self {
        CheckPayment(client, wallet_state)
    }
}
