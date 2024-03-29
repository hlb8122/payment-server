use std::fmt;

use actix_web::{error, HttpResponse};
use bitcoin::consensus::encode::Error as TxDeserializeError;
use bitcoincash_addr::AddressError;
use diesel::result::Error as DieselError;
use prost::DecodeError;

use crate::crypto::errors::CryptoError;

#[derive(Debug)]
pub enum ServerError {
    Crypto(CryptoError),
    NotFound,
    InvoiceRequestDecode,
    UnsupportedSigScheme,
    Payment(PaymentError),
    Address(AddressError),
    Diesel(DieselError),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            ServerError::Crypto(err) => return err.fmt(f),
            ServerError::NotFound => "not found",
            ServerError::InvoiceRequestDecode => "invoice request decoding error",
            ServerError::UnsupportedSigScheme => "signature scheme not supported",
            ServerError::Payment(err) => return err.fmt(f),
            ServerError::Address(err) => return err.fmt(f),
            ServerError::Diesel(err) => return err.fmt(f),
        };
        write!(f, "{}", printable)
    }
}

impl From<AddressError> for ServerError {
    fn from(err: AddressError) -> Self {
        ServerError::Address(err)
    }
}

impl From<CryptoError> for ServerError {
    fn from(err: CryptoError) -> Self {
        ServerError::Crypto(err)
    }
}

impl From<DecodeError> for ServerError {
    fn from(_: DecodeError) -> Self {
        ServerError::InvoiceRequestDecode
    }
}

impl From<DieselError> for ServerError {
    fn from(err: DieselError) -> Self {
        ServerError::Diesel(err)
    }
}

impl error::ResponseError for CryptoError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CryptoError::PubkeyDeserialization => HttpResponse::BadRequest(),
            CryptoError::SigDeserialization => HttpResponse::BadRequest(),
            CryptoError::Verification => HttpResponse::BadRequest(),
        }
        .body(self.to_string())
    }
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            // Do not yield sensitive information to clients
            ServerError::NotFound => HttpResponse::NotFound().body(self.to_string()),
            ServerError::InvoiceRequestDecode => HttpResponse::BadRequest().body(self.to_string()),
            ServerError::UnsupportedSigScheme => HttpResponse::BadRequest().body(self.to_string()),
            ServerError::Crypto(err) => err.error_response(),
            ServerError::Payment(err) => err.error_response(),
            ServerError::Address(err) => HttpResponse::BadRequest().body(err.to_string()),
            ServerError::Diesel(err) => HttpResponse::BadRequest().body(err.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum PaymentError {
    Content,
    Accept,
    Decode,
    Payload,
    NoMerchantDat,
    InvalidMerchantDat,
    InvalidAuth,
    NoToken,
    URIMalformed,
    NoTx,
    TxDeserialize(TxDeserializeError),
    InvalidOutputs,
    InvalidTx,
    MismatchedNetwork,
    AddrFetchFailed,
}

impl From<PaymentError> for ServerError {
    fn from(err: PaymentError) -> Self {
        ServerError::Payment(err)
    }
}

impl From<TxDeserializeError> for PaymentError {
    fn from(err: TxDeserializeError) -> PaymentError {
        PaymentError::TxDeserialize(err)
    }
}

impl fmt::Display for PaymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            PaymentError::Content => "invalid content-type",
            PaymentError::Accept => "not acceptable",
            PaymentError::Decode => "failed to decode body",
            PaymentError::Payload => "failed to receive payload",
            PaymentError::NoMerchantDat => "no merchant data",
            PaymentError::InvalidMerchantDat => "invalid merchant data",
            PaymentError::NoToken => "no token",
            PaymentError::InvalidAuth => "invalid authorization",
            PaymentError::URIMalformed => "malformed URI",
            PaymentError::NoTx => "no payment tx",
            PaymentError::TxDeserialize(_) => "payment tx malformed",
            PaymentError::InvalidOutputs => "invalid outputs",
            PaymentError::InvalidTx => "invalid tx",
            PaymentError::AddrFetchFailed => "failed to fetch address",
            PaymentError::MismatchedNetwork => "address mismatched with node network",
        };
        write!(f, "{}", printable)
    }
}

impl error::ResponseError for PaymentError {
    fn error_response(&self) -> HttpResponse {
        match self {
            PaymentError::Accept => HttpResponse::NotAcceptable(),
            PaymentError::Content => HttpResponse::UnsupportedMediaType(),
            PaymentError::NoMerchantDat => HttpResponse::BadRequest(),
            PaymentError::Payload => HttpResponse::BadRequest(),
            PaymentError::Decode => HttpResponse::BadRequest(),
            PaymentError::InvalidMerchantDat => HttpResponse::BadRequest(),
            PaymentError::InvalidAuth => HttpResponse::PaymentRequired(),
            PaymentError::NoToken => HttpResponse::PaymentRequired(),
            PaymentError::URIMalformed => HttpResponse::BadRequest(),
            PaymentError::NoTx => HttpResponse::BadRequest(),
            PaymentError::TxDeserialize(_) => HttpResponse::BadRequest(),
            PaymentError::InvalidOutputs => HttpResponse::BadRequest(),
            PaymentError::InvalidTx => HttpResponse::BadRequest(),
            PaymentError::MismatchedNetwork => HttpResponse::BadRequest(),
            PaymentError::AddrFetchFailed => HttpResponse::InternalServerError(),
        }
        .body(self.to_string())
    }
}
