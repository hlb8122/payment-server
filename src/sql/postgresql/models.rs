use super::schema::{payments, PaymentStateEnum};
use chrono::NaiveDateTime;
use diesel::*;
use uuid::Uuid;

#[derive(PartialEq, Debug, Serialize, Queryable, Deserialize)]
pub struct PaymentRow {
    pub id: Uuid,
    pub issue_time: NaiveDateTime,
    pub amount: i64,
    pub address: String,
    pub expiry_time: Option<NaiveDateTime>,
    pub req_memo: Option<String>,
    pub merchant_data: Option<Vec<u8>>,
    pub ack_memo: Option<String>,
    pub tokenize: bool,
    pub tx_data: Option<Vec<u8>>,
    pub payment_state: PaymentStateEnum,
    pub payment_time: Option<NaiveDateTime>,
    pub tx_id: Option<String>,
    pub refund_to: Option<String>,
    pub callback_url: Option<String>,
}

#[derive(Insertable, Queryable, Debug, PartialEq)]
#[table_name = "payments"]
pub struct NewPayment<'a> {
    pub id: &'a Uuid,
    pub issue_time: &'a NaiveDateTime,
    pub amount: i64,
    pub address: &'a str,
    pub expiry_time: Option<&'a NaiveDateTime>,
    pub req_memo: Option<&'a str>,
    pub merchant_data: Option<&'a [u8]>,
    pub tx_data: Option<&'a [u8]>,
    pub ack_memo: Option<&'a str>,
    pub payment_state: &'a PaymentStateEnum,
    pub tokenize: bool,
    pub callback_url: Option<&'a str>,
}
