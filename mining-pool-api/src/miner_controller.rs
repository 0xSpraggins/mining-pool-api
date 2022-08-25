use actix_web::HttpResponse;
use {

    crate::miner::*,
    crate::util::*,
};

// Get all miners
#[get("/miners")]
pub async fn get_miners() -> HttpResponse {
    // Get all the miner DAOs from the database and convert to a miner object
    let miners: Vec<Miner> = vec![];
    ResponseType::Ok(miners).get_response()
}

// Get miner by Id
#[get("/miners/{id}")]
pub async fn get_miner() -> HttpResponse {
    // Get miner DAO where miner id = requested value. Then convert to a miner object
    let miner: Option<Miner> = None;

    match miner {
        Some(miner) => ResponseType::Ok(miner).get_response(),
        None => ResponseType::NotFound(
            NotFoundResponse::new("Miner not found".to_string())
        ).get_response()
    }
}

// Create a new miner
#[post("/wallets/{id}/miners")]
pub async fn create_miner() -> HttpResponse {
    // Add miner to db after creating the miner DAO
    let miner: Vec<Miner> = vec![];
    ResponseType::Created(miner).get_response()

}