pub mod models;
pub mod schema;

use chrono::NaiveDateTime;
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error,
};
use uuid::Uuid;

use crate::{models::*, sql::postgresql::models::NewPayment};

pub fn add_payment(
    payment_details: &PaymentDetails,
    id: &Uuid,
    address: &str,
    amount: i64,
    req_memo: Option<&str>,
    ack_memo: Option<&str>,
    callback_url: Option<&str>,
    token_data: Option<&[u8]>,
    tx_data: Option<&[u8]>,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Uuid, Error> {
    use schema::{
        payments::dsl::{id as dsl_id, payments},
        PaymentStateEnum,
    };

    let issue_time = &NaiveDateTime::from_timestamp(payment_details.time as i64, 0);
    let expiry_time = payment_details
        .expires
        .map(|value| NaiveDateTime::from_timestamp(value as i64, 0));
    let merchant_data = payment_details
        .merchant_data
        .as_ref()
        .map(|value| &value[..]);

    // Construct row
    let new_payment = NewPayment {
        id,
        issue_time,
        amount,
        address,
        expiry_time: expiry_time.as_ref(),
        req_memo,
        merchant_data,
        ack_memo,
        token_data,
        tx_data,
        payment_state: &PaymentStateEnum::Pending,
        callback_url,
    };
    diesel::insert_into(payments)
        .values(&new_payment)
        .returning(dsl_id)
        .get_result(conn)
}
