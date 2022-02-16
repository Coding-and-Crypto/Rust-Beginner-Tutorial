use dotenv;
use reqwest;
use tokio;
use serde_json::Result;

use blockchain_parent::BlockchainInfo;
use blockchain_status::BlockchainStatus;
use blockchain_address::BlockchainAddress;
// use blockchain_transaction::BlockchainTransaction;


const HOST_ROOT: &str = "https://btcbook.nownodes.io/api/";


// Enum for dictating which type of request we have
pub enum BlockchainInfoRequest {
    STATUS,
    WALLET,
    TRANSACTION,
}

impl BlockchainInfoRequest {
    
    // Method to generate the associated url for each request type
    pub fn generate_url(request_type: BlockchainInfoRequest, 
                        wallet_address: Option<&str>, 
                        transaction_id: Option<&str>) -> BlockchainInfo {
        let host_root = String::from(HOST_ROOT);
        match request_type {
            BlockchainInfoRequest::STATUS => host_root + "",
            BlockchainInfoRequest::WALLET => host_root + "v2/address/" + wallet_address.expect("Wallet address is required"),
            BlockchainInfoRequest::TRANSACTION => host_root + "v2/tx/" + transaction_id.expect("Transaction ID is required"),
        }
    }

    // Method to point to which type of response we get after making the call
    pub fn parse_response(request_type: BlockchainInfoRequest, 
                        response: &str) -> String {
        let host_root = String::from(HOST_ROOT);
        match request_type {
            BlockchainInfoRequest::STATUS => {
                let json_response: BlockchainStatus = serde_json::from_str(response).expect("Failed to parse JSON");
                json_response
            },
            BlockchainInfoRequest::WALLET => {
                let json_response: BlockchainAddress = serde_json::from_str(response).expect("Failed to parse JSON");
                json_response
            },
            BlockchainInfoRequest::TRANSACTION => {
                let json_response: BlockchainStatus = serde_json::from_str(response).expect("Failed to parse JSON");
                json_response
            },
        }
    }
}


#[tokio::main]
pub async fn send_get_request(request_type: BlockchainInfoRequest, 
                            wallet_address: Option<&str>, 
                            transaction_id: Option<&str>) -> Result<()> {

    let url = BlockchainInfoRequest::generate_url(request_type, wallet_address, transaction_id);

    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .header("api-key", dotenv::var("API_KEY").expect("Error reading env var"))
        .send()
        .await
        .expect("Failed to get response")
        .text()
        .await
        .expect("Failed to convert to JSON");

    let output = BlockchainInfoRequest::parse_response(request_type, &res);
    println!("{:#?}", output);

    Ok(())
}