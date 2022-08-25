use {
    serde::{Deserialize, Serialize},
};


// JSON payload for miner
#[derive(Debug, Deserialize, Serialize)]
pub struct Miner {
    pub id: String,
    pub address: String,
    pub club_name: String,
    pub nickname: String,
    pub hash_rate: i32,
    // In MH/s
    pub shares_mined: i32,

}

// Post request body for adding miners
#[derive(Debug, Deserialize, Serialize)]
pub struct NewMinerRequest {
    nickname: String
}

// Database table record object
#[derive(Debug, Deserialize, Serialize)]
pub struct MinerDAO {
    pub id: String,
    pub address: String,
    pub nickname: String,
    pub hash_rate: i32,
    pub shares_mined: i32
}
