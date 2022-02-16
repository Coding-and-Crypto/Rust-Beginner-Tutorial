use {
    dotenv,
    reqwest,
    tokio,
    serde_json::Result,
    crate::blockchain_status::BlockchainStatus,
    crate::blockchain_address::BlockchainAddress,
};

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
                        transaction_id: Option<&str>) -> (BlockchainInfoRequest, String) {
        let host_root = String::from(HOST_ROOT);
        match request_type {
            BlockchainInfoRequest::STATUS => (request_type, host_root + ""),
            BlockchainInfoRequest::WALLET => (request_type, host_root + "v2/address/" + wallet_address.expect("Wallet address is required")),
            BlockchainInfoRequest::TRANSACTION => (request_type, host_root + "v2/tx/" + transaction_id.expect("Transaction ID is required")),
        }
    }

    // Method to point to which type of response we get after making the call
    pub fn parse_response(request_type: BlockchainInfoRequest, 
                        response: &str) {
        match request_type {
            BlockchainInfoRequest::STATUS => {
                let json_response: BlockchainStatus = serde_json::from_str(response).expect("Failed to parse JSON");
                println!("{:#?}", json_response);
            },
            BlockchainInfoRequest::WALLET => {
                let json_response: BlockchainAddress = serde_json::from_str(response).expect("Failed to parse JSON");
                println!("{:#?}", json_response);
            },
            BlockchainInfoRequest::TRANSACTION => {
                let json_response: BlockchainStatus = serde_json::from_str(response).expect("Failed to parse JSON");
                println!("{:#?}", json_response);
            },
        }
    }
}


#[tokio::main]
pub async fn send_get_request(request_type: BlockchainInfoRequest, 
                            wallet_address: Option<&str>, 
                            transaction_id: Option<&str>) -> Result<()> {

    let (request_type, url) = BlockchainInfoRequest::generate_url(request_type, wallet_address, transaction_id);

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

    BlockchainInfoRequest::parse_response(request_type, &res);

    Ok(())
}