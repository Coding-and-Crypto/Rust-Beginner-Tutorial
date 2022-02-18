#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vin {
    // txid: String,
    // vout: i64,
    // sequence: i64,
    // n: i64,
    pub addresses: Vec<String>,
    // is_address: bool,
    pub value: String,
    // hex: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vout {
    pub value: String,
    // n: i64,
    // hex: String,
    pub addresses: Vec<String>,
    // is_address: bool
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainTransaction {
    pub txid: String,
    // version: i64,
    // lock_time: i64,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    // block_hash: String,
    // block_height: i64,
    // confirmations: i64,
    // block_time: i64,
    // value: String,
    // value_in: String,
    // fees: String,
    // hex: String,
}