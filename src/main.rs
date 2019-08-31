#[macro_use]
extern crate clap;

pub mod bitcoin;
pub mod crypto;
pub mod net;
pub mod settings;

use std::io;

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use lazy_static::lazy_static;
use log::info;

use crate::{
    bitcoin::{BitcoinClient, WalletState},
    net::*,
    settings::Settings,
};

pub mod models {
    include!(concat!(env!("OUT_DIR"), "/models.rs"));
}

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("couldn't load config");
}

fn main() -> io::Result<()> {
    let sys = actix_rt::System::new("bip70-server");

    // Init logging
    env_logger::from_env(Env::default().default_filter_or("actix_web=info,bip70-server=info"))
        .init();
    info!("starting public endpoint @: {}", SETTINGS.bind_public);
    info!("starting private endpoint @: {}", SETTINGS.bind_private);

    // Init wallet
    let wallet_state = WalletState::default();

    // Init Bitcoin client
    let bitcoin_client = BitcoinClient::new(
        format!("http://{}:{}", SETTINGS.node_ip.clone(), SETTINGS.rpc_port),
        SETTINGS.rpc_username.clone(),
        SETTINGS.rpc_password.clone(),
    );

    // Init ZMQ
    // let (tx_stream, connection) =
    //     tx_stream::get_tx_stream(&format!("tcp://{}:{}", SETTINGS.node_ip, SETTINGS.zmq_port));
    // let key_stream = tx_stream::extract_details(tx_stream);
    // actix_rt::Arbiter::current().send(connection.map_err(|e| error!("{:?}", e)));

    let bitcoin_client_inner = bitcoin_client.clone();
    let wallet_state_inner = wallet_state.clone();
    let public_endpoint = HttpServer::new(move || {
        // Init app
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                // Payment endpoint
                web::resource("/payments")
                    .data((
                        bitcoin_client_inner.to_owned(),
                        wallet_state_inner.to_owned(),
                    ))
                    .route(web::post().to_async(payment_handler)),
            )
    });

    let private_endpoint = HttpServer::new(move || {
        // Init app
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                // Payment endpoint
                web::resource("/invoice")
                    .data((bitcoin_client.to_owned(), wallet_state.to_owned()))
                    .route(web::post().to_async(generate_invoice)),
            )
    });

    sys.run()
}
