#[macro_use]
extern crate serde_derive;

mod blockchain_info;
mod blockchain_status;
mod blockchain_address;
mod blockchain_transaction;

use crate::blockchain_status::BlockchainStatus;

fn blockchain_info_app() {
    let blockchain_status: BlockchainStatus = blockchain_info::blockchain_status_request();
    println!("Querying {} chain: {}", blockchain_status.blockbook.coin, blockchain_status.backend.chain)
    // blockchain_info::send_get_request(BlockchainInfoRequest::WALLET, Some("34kPpdiw6RE7j3ygN3W7Z3JF2gF1PSuWZa"), None);
    // blockchain_info::send_get_request(BlockchainInfoRequest::TRANSACTION, None, Some("d83cae367010766919b933f54122db87834e0fe50e50e78748aba06141a16eff"));
}

fn main() {
    blockchain_info_app();
}