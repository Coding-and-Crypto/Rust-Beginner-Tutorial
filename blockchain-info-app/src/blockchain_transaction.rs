#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vin {
    txid: String,
    vout: i64,
    sequence: i64,
    n: i64,
    addresses: Vec<String>,
    is_address: bool,
    value: String,
    hex: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vout {
    value: String,
    n: i64,
    hex: String,
    addresses: Vec<String>,
    is_address: bool
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainTransaction {
    txid: String,
    version: i64,
    // lock_time: i64,
    vin: Vec<Vin>,
    vout: Vec<Vout>,
    block_hash: String,
    block_height: i64,
    confirmations: i64,
    block_time: i64,
    value: String,
    value_in: String,
    fees: String,
    hex: String,
}