use actix_web::HttpResponse;
use actix_web::web::{Data, Json, Path};
use uuid::Uuid;
use {

    crate::miner::*,
    crate::util::*,
};
use crate::{DBPool, DBPooledConnection};

// Get all miners
#[get("/miners")]
pub async fn get_miners(pool: Data<DBPool>) -> HttpResponse {
    // Get all the miner DAOs from the database and convert to a miner object
    let conn = crate::get_connection_to_pool(pool);
    let miners: Vec<Miner> = get_all_miners(&conn);
    ResponseType::Ok(miners).get_response()
}

// Get miner by Id
#[get("/miners/{id}")]
pub async fn get_miner(path: Path<(String,)>,pool: Data<DBPool>) -> HttpResponse {
    // Get miner DAO where miner id = requested value. Then convert to a miner object
    let conn = crate::get_connection_to_pool(pool);
    let miner: Option<Miner> = get_miner_by_id(
        Uuid::parse_str(path.0.0.as_str()).unwrap(), &conn);
    match miner {
        Some(miner) => ResponseType::Ok(miner).get_response(),
        None => ResponseType::NotFound(
            NotFoundResponse::new("Miner not found".to_string())
        ).get_response()
    }
}

// Create a new miner
#[post("/wallets/{id}/miners")]
pub async fn create_miner(path: Path<(String,)>,
                          miner_request: Json<NewMinerRequest>,
                          pool: Data<DBPool>) -> HttpResponse {
    // Add miner to db after creating the miner DAO
    let conn = crate::get_connection_to_pool(pool);
    match create_new_miner(miner_request.0, Uuid::parse_str(path.0.0.as_str()).unwrap(), &conn) {
        Ok(miner) => ResponseType::Created(miner).get_response(),
        Err(_) => ResponseType::NotFound(
            NotFoundResponse::new("Error creating miner".to_string())
        ).get_response(),
    }

}