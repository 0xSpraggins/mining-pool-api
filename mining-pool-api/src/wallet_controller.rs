use {
    actix_web::HttpResponse,
    actix_web::web::{Data, Json, Path},
    uuid::Uuid,

    crate::wallet::*,
    crate::util::*,
    crate::DBPool
};


// Get all miners
#[get("/wallets")]
pub async fn get_wallets(pool: Data<DBPool>) -> HttpResponse {
    // Get all the wallet DAOs from the database and convert to a wallet object
    let conn = crate::get_connection_to_pool(pool);
    let wallets: Vec<Wallet> = get_all_wallets(&conn);
    ResponseType::Ok(wallets).get_response()
}

// Get miner by Id
#[get("/wallets/{id}")]
pub async fn get_wallet(path: Path<(String,)>,pool: Data<DBPool>) -> HttpResponse {
    // Get miner DAO where wallet id = requested value. Then convert to a wallet object
    let conn = crate::get_connection_to_pool(pool);
    let wallet: Option<Wallet> = get_wallet_by_id(
        Uuid::parse_str(path.0.0.as_str()).unwrap(), &conn);
    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),
        None => ResponseType::NotFound(
            NotFoundResponse::new("Wallet not found".to_string())
        ).get_response()
    }
}

// Create a new wallet
#[post("/wallets")]
pub async fn create_wallet(wallet_request: Json<NewWalletRequest>, pool: Data<DBPool>) -> HttpResponse {
    // Add miner to db after creating the miner DAO
    let conn = crate::get_connection_to_pool(pool);
    match create_new_wallet(wallet_request.0, &conn) {
        Ok(wallet) => ResponseType::Created(wallet).get_response(),
        Err(_) => ResponseType::NotFound(
            NotFoundResponse::new("Error creating wallet".to_string())
        ).get_response(),
    }

}