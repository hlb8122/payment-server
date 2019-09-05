use diesel::{
    deserialize::{self, FromSql},
    pg::Pg,
    serialize::{self, IsNull, Output, ToSql},
    *,
};
use std::io::Write;

#[derive(SqlType)]
#[postgres(type_name = "payment_state_type")]
pub struct PaymentStateType;

#[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression, Deserialize, Serialize)]
#[sql_type = "PaymentStateType"]
pub enum PaymentStateEnum {
    Pending,
    Received,
    Confirmed,
    Rejected,
    Expired,
}

impl ToSql<PaymentStateType, Pg> for PaymentStateEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            Self::Pending => out.write_all(b"pending")?,
            Self::Received => out.write_all(b"received")?,
            Self::Confirmed => out.write_all(b"confirmed")?,
            Self::Rejected => out.write_all(b"rejected")?,
            Self::Expired => out.write_all(b"expired")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<PaymentStateType, Pg> for PaymentStateEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"pending" => Ok(Self::Pending),
            b"received" => Ok(Self::Received),
            b"confirmed" => Ok(Self::Confirmed),
            b"rejected" => Ok(Self::Rejected),
            b"expired" => Ok(Self::Expired),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

table! {
    use diesel::sql_types::*;
    use super::PaymentStateType;
    payments (id) {
        id -> Uuid, // Payment ID
        issue_time -> Timestamp, // The time of issuance
        amount -> BigInt, // The amount to be paid
        address -> Text, // Address to be paid to
        expiry_time -> Nullable<Timestamp>, // Expiry time of the payment
        req_memo -> Nullable<Text>, // Memo to be included in the request
        merchant_data -> Nullable<Blob>, // Merchant data
        ack_memo -> Nullable<Text>, // Memo to be included in the request
        token_data -> Nullable<Blob>, // Token to be signed then attached to payment ack response
        tx_data -> Nullable<Blob>, // Data required inside OP_RETURN
        payment_state -> PaymentStateType, // Payment state
        payment_time -> Nullable<Timestamp>, // Time payment was completed
        tx_id -> Nullable<Text>, // Transaction ID of the payment
        refund_to -> Nullable<Text>, // Refund address
        callback_url -> Nullable<Text>, // Callback URL
    }
}
