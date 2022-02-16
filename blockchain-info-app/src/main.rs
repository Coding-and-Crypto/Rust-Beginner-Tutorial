#[macro_use]
extern crate serde_derive;

use serde_json::Result;
use error_chain::error_chain;
use std::io::Read;
use reqwest;
use tokio;


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Blockbook {
    coin: String,
    host: String,
    version: String,
    git_commit: String,
    build_time: String,
    sync_mode: bool,
    initial_sync: bool,
    in_sync: bool,
    best_height: i64,
    last_block_time: String,
    in_sync_mempool: bool,
    last_mempool_time: String,
    mempool_size: i64,
    decimals: i64,
    db_size: i64,
    about: String
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Backend {
    chain: String,
    blocks: i64,
    headers: i64,
    best_block_hash: String,
    difficulty: String,
    size_on_disk: i64,
    version: String,
    subversion: String,
    protocol_version: String
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BlockchainStatus {
    blockbook: Blockbook,
    backend: Backend,
}


#[tokio::main]
async fn status() -> Result<()> {

    // let tx_id = "d83cae367010766919b933f54122db87834e0fe50e50e78748aba06141a16eff";
    // let mut url = "https://btcbook.nownodes.io/api/v2/tx/".to_string();
    // url += tx_id;

    let client = reqwest::Client::new();
    let res = client
        .get("https://btcbook.nownodes.io/api/")
        .header("api-key", "5gI430GApahtEPjKzqUZd8FyCW9kVnvb")
        .send()
        .await
        .expect("Failed to get response")
        .text()
        .await
        .expect("Failed to convert to JSON");

    let status: BlockchainStatus = serde_json::from_str(&res).expect("Failed to parse JSON");
    println!("{:#?}", status);

    Ok(())
}

fn main() {
    status();
    // deserialize();
}