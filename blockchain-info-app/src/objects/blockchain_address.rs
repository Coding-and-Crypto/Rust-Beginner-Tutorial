#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BlockchainAddress {
    page: u16,
    totalPages: u16,
    itemsOnPage: u64,
    address: String,
    balance: String,
    totalReceived: String,
    totalSent: String,
    unconfirmedBalance: String,
    unconfirmedTxs: u64,
    txs: u64,
    txids: Vec<String>
}

impl BlockchainInfo for BlockchainAddress {
    
}