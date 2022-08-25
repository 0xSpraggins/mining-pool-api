use actix_web::HttpResponse;
use {

    crate::wallet::*,
    crate::util::*,
};

// Get all miners
#[get("/wallets")]
pub async fn get_wallets() -> HttpResponse {
    // Get all the wallet DAOs from the database and convert to a wallet object
    let wallets: Vec<Wallet> = vec![];
    ResponseType::Ok(wallets).get_response()
}

// Get miner by Id
#[get("/wallets/{id}")]
pub async fn get_wallet() -> HttpResponse {
    // Get miner DAO where wallet id = requested value. Then convert to a wallet object
    let wallet: Option<Wallet> = None;

    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),
        None => ResponseType::NotFound(
            NotFoundResponse::new("Wallet not found".to_string())
        ).get_response()
    }
}

// Create a new wallet
#[post("/wallets")]
pub async fn create_wallet() -> HttpResponse {
    // Add miner to db after creating the miner DAO
    let wallet: Vec<Wallet> = vec![];
    ResponseType::Created(wallet).get_response()

}