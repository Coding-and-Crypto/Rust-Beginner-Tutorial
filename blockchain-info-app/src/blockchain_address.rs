#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainAddress {
    page: u16,
    total_pages: u16,
    items_on_page: u64,
    address: String,
    balance: String,
    total_received: String,
    total_sent: String,
    unconfirmed_balance: String,
    unconfirmed_txs: u64,
    txs: u64,
    txids: Vec<String>
}