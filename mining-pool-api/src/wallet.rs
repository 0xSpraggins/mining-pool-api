use {
    serde::{Deserialize, Serialize},
    crate::miner::*,
};

// JSON payload for wallet
#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub address: String,
    pub club_name: String,
    pub total_hash_rate: i32,
    pub total_shares_mined: i32,
    pub total_workers_online: i32,
    pub workers_online: Vec<Miner>,
}

// Post request body for new wallet
#[derive(Debug, Deserialize, Serialize)]
pub struct NewWalletRequest {
    club_name: String,
}

// Data Access Object
#[derive(Debug, Deserialize, Serialize)]
pub struct WalletDAO {
    pub address: String,
    pub club_name: String,
}