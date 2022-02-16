// #[macro_use]
// extern crate serde_derive;

mod blockchain_info;
mod blockchain_parent;
mod blockchain_status;
mod blockchain_address;
// mod blockchain_transaction;

use blockchain_info::BlockchainInfoRequest;

fn main() {
    blockchain_info::send_get_request(BlockchainInfoRequest::STATUS, None, None);
    // blockchain_status::send_get_request(BlockchainInfoRequest::WALLET, "34kPpdiw6RE7j3ygN3W7Z3JF2gF1PSuWZa", None);
    // blockchain_status::send_get_request(BlockchainInfoRequest::TRANSACTION, None, "d83cae367010766919b933f54122db87834e0fe50e50e78748aba06141a16eff");
}