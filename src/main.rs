#[macro_use]
extern crate clap;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

pub mod bitcoin;
pub mod crypto;
pub mod net;
pub mod settings;
pub mod sql;

use std::io;

use actix_http::HttpService;
use actix_web::{dev::Server, middleware::Logger, web, App};
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use env_logger::Env;
use lazy_static::lazy_static;
use log::info;

use crate::{bitcoin::BitcoinClient, net::*, settings::Settings};

pub mod models {
    include!(concat!(env!("OUT_DIR"), "/models.rs"));
}

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("couldn't load config");
}

pub type ConnPool = Pool<ConnectionManager<PgConnection>>;

fn main() -> io::Result<()> {
    let sys = actix_rt::System::new("bip70-server");

    // Init logging
    env_logger::from_env(Env::default().default_filter_or("actix_web=info,bip70-server=info"))
        .init();
    info!("starting public endpoint @: {}", SETTINGS.bind_public);
    info!("starting private endpoint @: {}", SETTINGS.bind_private);

    // Init Bitcoin client
    let bitcoin_client = BitcoinClient::new(
        format!("http://{}:{}", SETTINGS.node_ip.clone(), SETTINGS.rpc_port),
        SETTINGS.rpc_username.clone(),
        SETTINGS.rpc_password.clone(),
    );

    // Init SQL connection
    let url = format!(
        "{}://{}:{}@{}:{}/{}",
        SETTINGS.sql.prefix,
        SETTINGS.sql.username,
        SETTINGS.sql.password,
        SETTINGS.sql.host,
        SETTINGS.sql.port,
        SETTINGS.sql.db
    );
    let manager = ConnectionManager::<PgConnection>::new(url);
    let pool = Pool::builder()
        .build(manager)
        .expect("failed to create pool");

    // Init ZMQ
    // TODO: Check confirmations
    // let (tx_stream, connection) =
    //     tx_stream::get_tx_stream(&format!("tcp://{}:{}", SETTINGS.node_ip, SETTINGS.zmq_port));
    // let key_stream = tx_stream::extract_details(tx_stream);
    // actix_rt::Arbiter::current().send(connection.map_err(|e| error!("{:?}", e)));

    let bitcoin_client_inner = bitcoin_client.clone();
    let pool_inner = pool.clone();

    Server::build()
        .bind("public", &SETTINGS.bind_public, move || {
            HttpService::build().finish(
                // Init app
                App::new()
                    .wrap(Logger::default())
                    .wrap(Logger::new("%a %{User-Agent}i"))
                    .service(
                        // Payment route
                        web::resource("/payment/{payment_id}")
                            .data((bitcoin_client_inner.to_owned(), pool_inner.to_owned()))
                            .route(web::post().to_async(payment_handler)),
                    ),
            )
        })
        .unwrap()
        .bind("private", &SETTINGS.bind_private, move || {
            HttpService::build().finish(
                // Init app
                App::new()
                    .wrap(Logger::default())
                    .wrap(Logger::new("%a %{User-Agent}i"))
                    .service(
                        // Create invoice route
                        web::resource("/invoice")
                            .data((bitcoin_client.to_owned(), pool.to_owned()))
                            .route(web::post().to_async(generate_invoice)),
                    ),
            )
        })
        .unwrap()
        .run();

    sys.run()
}
