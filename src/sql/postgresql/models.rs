use super::schema::{payments, PaymentStateEnum};
use chrono::NaiveDateTime;
use diesel::*;

#[derive(PartialEq, Debug, Serialize, Queryable, Deserialize)]
pub struct Payment {
    pub issue_time: NaiveDateTime,
    pub amount: i32,
    pub address: String,
    pub expiry_time: Option<NaiveDateTime>,
    pub state: PaymentStateEnum,
    pub payment_time: Option<NaiveDateTime>,
    pub token: Option<String>,
    pub callback_url: Option<String>,
}

#[derive(Insertable, Queryable, Debug, PartialEq)]
#[table_name = "payments"]
pub struct NewPayment<'a> {
    pub issue_time: &'a NaiveDateTime,
    pub amount: &'a i32,
    pub address: &'a str,
    pub expiry_time: Option<&'a NaiveDateTime>,
    pub state: &'a PaymentStateEnum,
    pub payment_time: Option<&'a NaiveDateTime>,
    pub token: Option<&'a str>,
    pub callback_url: Option<&'a str>,
}
