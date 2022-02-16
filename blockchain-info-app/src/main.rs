#[macro_use]
extern crate serde_derive;

mod blockchain_info;
mod blockchain_status;
mod blockchain_address;
mod blockchain_transaction;

use crate::blockchain_status::BlockchainStatus;
use crate::blockchain_address::BlockchainAddress;

use dotenv;
use std::{thread, time};

fn blockchain_info_app(address: &str) {
    let blockchain_status: BlockchainStatus = blockchain_info::blockchain_status_request();
    println!("\n\nQuerying {} - chain: {}\n\n", &blockchain_status.blockbook.coin, &blockchain_status.backend.chain);
    let blockchain_address: BlockchainAddress = blockchain_info::blockchain_address_request(address);
    println!("\n\nAnalyzing transactions for Bitcoin address {}", &blockchain_address.address);
    let sleep_time = time::Duration::from_millis(2500);
    thread::sleep(sleep_time);
    println!("\nWe will look up the following transactions:\n");
    thread::sleep(sleep_time);
    println!("{:?}", &blockchain_address.txids);
    // blockchain_info::send_get_request(BlockchainInfoRequest::TRANSACTION, None, Some("d83cae367010766919b933f54122db87834e0fe50e50e78748aba06141a16eff"));
}

fn main() {
    let entered_address = dotenv::var("WALLET").expect("Error reading env var");
    blockchain_info_app(&entered_address);
}