// extern crate reqwest;
// extern crate tokio;

#[macro_use]
extern crate serde_derive;

use serde_json::Result;
use error_chain::error_chain;
use std::io::Read;
use reqwest;
use tokio;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }

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

fn deserialize() -> Result<()> {
    let data = r#"
    {
        "blockbook": {
            "coin": "Bitcoin",
            "host": "234b52430ba9",
            "version": "0.3.5",
            "gitCommit": "eee7f67",
            "buildTime": "2021-09-07T07:18:33+00:00",
            "syncMode": true,
            "initialSync": false,
            "inSync": true,
            "bestHeight": 723518,
            "lastBlockTime": "2022-02-16T01:06:04.097312702Z",
            "inSyncMempool": true,
            "lastMempoolTime": "2022-02-16T01:14:33.770889316Z",
            "mempoolSize": 1349,
            "decimals": 8,
            "dbSize": 357357521388,
            "about": "Blockbook - blockchain indexer for Trezor wallet https://trezor.io/. Do not use for any other purpose."
        },
        "backend": {
            "chain": "main",
            "blocks": 723518,
            "headers": 723518,
            "bestBlockHash": "00000000000000000001bd75490cfd837723762930d6f120a5eefd05ac2bcc86",
            "difficulty": "26690525287405.5",
            "sizeOnDisk": 444231090866,
            "version": "220000",
            "subversion": "/Satoshi:22.0.0/",
            "protocolVersion": "70016"
        }
    }
    "#;
    let status: BlockchainStatus = serde_json::from_str(data).expect("Failed to parse JSON");
    println!("{:#?}", status);

    Ok(())
}

// #[tokio::main]
// async fn get_status() -> Result<()> {

//     // let tx_id = "d83cae367010766919b933f54122db87834e0fe50e50e78748aba06141a16eff";
//     // let mut url = "https://btcbook.nownodes.io/api/v2/tx/".to_string();
//     // url += tx_id;

//     let client = reqwest::Client::new();
//     let res = client
//         .get("https://btcbook.nownodes.io/api/")
//         .header("api-key", "5gI430GApahtEPjKzqUZd8FyCW9kVnvb")
//         .send()
//         .await
//         .expect("Failed to get response");

//     let body = res.text().await?;

//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());
//     println!("{}", body);

//     println!("\n\n---------------------\n\n");

//     let status: BlockchainStatus = serde_json::from_str(res.json().await?)?;

//     Ok(())
// }

fn main() {
    // println!("{}", get_status());
    deserialize();
}