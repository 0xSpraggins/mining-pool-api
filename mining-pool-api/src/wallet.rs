use {
    diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl},
    diesel::query_dsl::methods::FilterDsl,
    diesel::result::Error,
    rand::Rng,
    serde::{Deserialize, Serialize},
    uuid::Uuid,

    super::schema::wallets,

    crate::DBPooledConnection,
    crate::miner::Miner,
};
use crate::miner::MinerDAO;


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

impl Wallet {
    pub fn to_wallet_dao(&self) -> WalletDAO {
        WalletDAO {
            address: Uuid::parse_str(self.address.as_str()).unwrap(),
            club_name: self.club_name.to_string(),
        }
    }
}

// Post request body for new wallet
#[derive(Debug, Deserialize, Serialize)]
pub struct NewWalletRequest {
    club_name: String,
}

// Data Access Object
#[derive(Queryable, Insertable)]
#[diesel(table_name = wallets)]
pub struct WalletDAO {
    pub address: Uuid,
    pub club_name: String,
}

impl WalletDAO {
    pub fn to_wallet(&self, workers_online: Vec<Miner>) -> Wallet {
        Wallet {
            address: self.address.to_string(),
            club_name: self.club_name.to_string(),
            total_hash_rate: workers_online.iter().map(|w| w.hash_rate).sum(),
            total_shares_mined: workers_online.iter().map(|w| w.shares_mined).sum(),
            total_workers_online: workers_online.iter.len() as i32,
            workers_online,
        }
    }
}

pub fn get_workers_online(_wallet_dao: &WalletDAO, conn: &DBPooledConnection) -> Vec<Miner> {
    use crate::schema::miners::dsl::*;

    match miners.filter(address.eq(_wallet_dao.address)).load::<MinerDAO>(conn) {
        Ok(result) => result
            .into_iter()
            .map(|m| m.to_miner(_wallet_dao.club_name.clone()))
            .collect::<Vec<Miner>>(),
        Err(_) => vec![],
    }

}

pub fn get_wallet_by_id(_address: Uuid, conn: &DBPooledConnection) -> Option<Wallet> {
    use crate::schema::wallets::dsl::*;

    match wallets.filter(address.eq(_address)).load::<WalletDAO>(conn) {
        Ok(result) => {
            match result.first() {
                Some(wallet) => {
                    let workers_online = get_workers_online(&wallet, conn);
                    Some(wallet.to_wallet(workers_online))
                },
                _ => None,
            }
        },
        Err(_) => None,
    }
}

pub fn get_all_wallets(conn: &DBPooledConnection) -> Vec<Wallet> {
    use crate::schema::wallets::dsl::*;
    match wallets.load::<WalletDAO>(conn) {
        Ok(result) => {
            result.into_iter().map(|m| {
                let club_name = m.get_club_name(m.address, conn);
                m.to_mine(club_name)
            }).collect::<Vec<Miner>>()
        },
        Err(_) => vec![],
    }
}

pub fn create_new_wallet(new_wallet_request: NewWalletRequest, conn: &DBPooledConnection) -> Result<Wallet, Error> {
    use crate::schema::wallets::dsl::*;
    let new_wallet = Wallet {
        address: Uuid::new_v4().to_string(),
        club_name: new_wallet_request.club_name.to_string(),
        total_hash_rate: 0,
        total_shares_mined: 0,
        total_workers_online: 0,
        workers_online: vec![]
    };
    let new_wallet_dao = new_wallet.to_wallet_dao();
    match diesel::insert_into(wallets).values(&new_wallet).execute(conn) {
        Ok(_) => Ok(new_wallet_dao.to_wallet(vec![])),
        Err(e) => Err(e),
    }
}