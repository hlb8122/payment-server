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
}

impl ToSql<PaymentStateType, Pg> for PaymentStateEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            Self::Pending => out.write_all(b"pending")?,
            Self::Received => out.write_all(b"received")?,
            Self::Confirmed => out.write_all(b"confirmed")?,
            Self::Rejected => out.write_all(b"rejected")?,
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
        amount -> Integer, // The amount to be paid
        address -> Text, // Address to be paid to
        expiry_time -> Nullable<Timestamp>, // Expiry time of the payment
        merchant_data -> Nullable<Blob>, // Merchant data
        state -> PaymentStateType, // Payment state
        payment_time -> Nullable<Timestamp>, // Time payment was completed
        token -> Nullable<Blob>, // Token to be signed then attached to payment ack response
        callback_url -> Nullable<Text>, // Callback URL
    }
}
