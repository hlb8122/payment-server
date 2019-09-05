mod client;
// pub mod tx_stream;

use std::string::ToString;

use bitcoin::{Transaction, TxOut};
use serde::Deserialize;

use crate::models::Output;

pub use client::BitcoinClient;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum Network {
    Mainnet = 0,
    Testnet = 1,
    Regnet = 2,
}

impl From<bitcoincash_addr::Network> for Network {
    fn from(network: bitcoincash_addr::Network) -> Network {
        match network {
            bitcoincash_addr::Network::Main => Network::Mainnet,
            bitcoincash_addr::Network::Test => Network::Testnet,
            bitcoincash_addr::Network::Regtest => Network::Regnet,
        }
    }
}

impl Into<bitcoincash_addr::Network> for Network {
    fn into(self) -> bitcoincash_addr::Network {
        match self {
            Network::Mainnet => bitcoincash_addr::Network::Main,
            Network::Testnet => bitcoincash_addr::Network::Test,
            Network::Regnet => bitcoincash_addr::Network::Regtest,
        }
    }
}

impl ToString for Network {
    fn to_string(&self) -> String {
        match self {
            Network::Mainnet => "mainnet".to_string(),
            Network::Testnet => "testnet".to_string(),
            Network::Regnet => "regnet".to_string(),
        }
    }
}

pub fn check_p2pkh(output: &TxOut, expected_amount: u64, expected_pk_hash: &[u8]) -> bool {
    // Check first output
    if output.value != expected_amount {
        return false;
    }

    // Check p2pkh addr
    let script = &output.script_pubkey[..];
    if let Some(pubkey_hash) = extract_pubkey_hash(script) {
        expected_pk_hash == &pubkey_hash[..]
    } else {
        false
    }
}

pub fn check_outputs(tx: Transaction, expected_amount: u64, expected_pk_hash: &[u8]) -> bool {
    // TODO: Enforce op_return outputs
    tx.output
        .iter()
        .any(|output| check_p2pkh(output, expected_amount, expected_pk_hash))
}

pub fn extract_op_return(script: &[u8]) -> Option<Vec<u8>> {
    // OP_RETURN || LEN || keyserver || bitcoin pk hash || peer host
    if script.len() <= 2 + 9 + 20 {
        // Too short
        return None;
    }

    if script[0] != 106 {
        // Not op_return
        return None;
    }

    if script[1] as usize != script.len() - 2 {
        // Not length
        return None;
    }

    Some(script[2..script.len()].to_vec())
}

fn extract_pubkey_hash(raw_script: &[u8]) -> Option<Vec<u8>> {
    if raw_script.len() != 25 {
        return None;
    }

    if raw_script[0..3] != [118, 169, 20] {
        return None;
    }

    if raw_script[23..25] != [136, 172] {
        return None;
    }

    Some(raw_script[3..23].to_vec())
}

pub fn generate_outputs(pk_hash: &[u8], amount: u64, data: &[u8]) -> Vec<Output> {
    // Generate p2pkh
    let p2pkh_script_pre: [u8; 3] = [118, 169, 20];
    let p2pkh_script_post: [u8; 2] = [136, 172];
    let p2pkh_script = [&p2pkh_script_pre[..], &pk_hash[..], &p2pkh_script_post[..]].concat();
    let p2pkh_output = Output {
        amount: Some(amount),
        script: p2pkh_script,
    };

    let op_return_script = [&[106, 9 + 20 + data.len() as u8][..], &data].concat();
    let op_return = Output {
        amount: Some(0),
        script: op_return_script,
    };

    vec![p2pkh_output, op_return]
}
