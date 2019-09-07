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

use crate::{
    models::*,
    sql::postgresql::{
        models::{NewPayment, PaymentRow},
        schema::PaymentStateEnum,
    },
};

use schema::payments::dsl::{self, payments};

pub fn add_payment(
    payment_details: &PaymentDetails,
    id: &Uuid,
    address: &str,
    amount: i64,
    req_memo: Option<&str>,
    ack_memo: Option<&str>,
    tokenize: bool,
    tx_data: Option<&[u8]>,
    callback_url: Option<&str>,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Uuid, Error> {
    use schema::{payments::dsl::id as dsl_id};

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
        tokenize,
        tx_data,
        payment_state: &PaymentStateEnum::Pending,
        callback_url,
    };
    diesel::insert_into(payments)
        .values(&new_payment)
        .returning(dsl_id)
        .get_result(conn)
}

pub fn get_payment(
    payment_id: &str,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<PaymentRow, Error> {
    use schema::payments::dsl::id as dsl_id;

    let uuid_payment_id = Uuid::parse_str(&payment_id).unwrap();
    payments
        .filter(dsl_id.eq(uuid_payment_id))
        .first::<models::PaymentRow>(conn)
}

pub fn reject_payment(
    payment_id: &str,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<(), Error> {
    let uuid_payment_id = Uuid::parse_str(&payment_id).unwrap();
    diesel::update(payments.find(uuid_payment_id))
        .set(dsl::payment_state.eq(PaymentStateEnum::Rejected))
        .execute(conn)?;
    Ok(())
}

pub fn accept_payment(
    payment_id: &str,
    tx_id: &str,
    reund_to: Option<&str>,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<(), Error> {
    let uuid_payment_id = Uuid::parse_str(&payment_id).unwrap();
    let gen_accept_time = Utc::now().naive_utc();
    diesel::update(payments.find(uuid_payment_id))
        .set((
            dsl::payment_state.eq(PaymentStateEnum::Received),
            dsl::payment_time.eq(gen_accept_time),
            dsl::refund_to.eq(reund_to),
            dsl::tx_id.eq(tx_id),
        ))
        .execute(conn)?;
    Ok(())
}
