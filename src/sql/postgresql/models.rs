use super::schema::{payments, PaymentStateEnum};
use chrono::NaiveDateTime;
use diesel::*;
use uuid::Uuid;

#[derive(PartialEq, Debug, Serialize, Queryable, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub issue_time: NaiveDateTime,
    pub amount: i64,
    pub address: String,
    pub expiry_time: Option<NaiveDateTime>,
    pub merchant_data: Option<Vec<u8>>,
    pub state: PaymentStateEnum,
    pub payment_time: Option<NaiveDateTime>,
    pub token: Option<Vec<u8>>,
    pub callback_url: Option<String>,
}

#[derive(Insertable, Queryable, Debug, PartialEq)]
#[table_name = "payments"]
pub struct NewPayment<'a> {
    pub id: &'a Uuid,
    pub issue_time: &'a NaiveDateTime,
    pub amount: &'a i64,
    pub address: &'a str,
    pub expiry_time: Option<&'a NaiveDateTime>,
    pub merchant_data: Option<&'a [u8]>,
    pub state: &'a PaymentStateEnum,
    pub token: Option<&'a [u8]>,
    pub callback_url: Option<&'a str>,
}
