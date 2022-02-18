#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainAddress {
    // page: i64,
    // total_pages: i64,
    // items_on_page: i64,
    pub address: String,
    // balance: String,
    // total_received: String,
    // total_sent: String,
    // unconfirmed_balance: String,
    // unconfirmed_txs: i64,
    // txs: i64,
    pub txids: Vec<String>
}