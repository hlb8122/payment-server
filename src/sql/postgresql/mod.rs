pub mod models;
pub mod schema;

use chrono::{NaiveDateTime, Utc};
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error,
};
use uuid::Uuid;

use crate::{models::*, sql::postgresql::models::NewPayment, ConnPool};

pub fn add_payment(
    invoice_params: &InvoiceParams,
    str_addr: &str,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Uuid, Error> {
    use schema::{payments::dsl::*, PaymentStateEnum};

    let dt_issue_time = NaiveDateTime::from_timestamp(invoice_params.time as i64, 0);
    let dt_expiry_time = match invoice_params.expiry {
        0 => None,
        value => Some(NaiveDateTime::from_timestamp(value as i64, 0)),
    };

    let opt_callback_url = match invoice_params.callback_url.as_str() {
        "" => None,
        value => Some(value),
    };

    let opt_merchant_data = if invoice_params.merchant_data.is_empty() {
        None
    } else {
        Some(&invoice_params.merchant_data[..])
    };

    // Construct row
    let new_payment = NewPayment {
        id: &Uuid::new_v4(),
        issue_time: &dt_issue_time,
        amount: &(invoice_params.amount as i32),
        address: &str_addr,
        expiry_time: dt_expiry_time.as_ref(),
        merchant_data: opt_merchant_data,
        state: &PaymentStateEnum::Pending,
        token: Some(&invoice_params.token),
        callback_url: opt_callback_url,
    };
    diesel::insert_into(payments)
        .values(&new_payment)
        .returning(id)
        .get_result(conn)
}
