use crate::models::network_model::{Network, NetworkManager};
use crate::models::user_model::User;
use crate::models::wallet_model::Wallet;
use crate::services::db::Database;
use crate::utils::api_response::ApiResponse;

use actix_web::{
    delete,
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    patch, post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubmitCreateNetwork {
    pub network_type: String,
    pub chain_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubmitDeleteNetwork {
    pub network_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubmitSetLastBlock {
    pub network_type: String,
    pub last_scanned_block: u128,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubmitUpdateNetwork {
    pub old_chain_id: String,
    pub new_chain_id: String,
}

macro_rules! try_or_return_string {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(e) => return ApiResponse::new_from_macro(e),
        }
    };
}

macro_rules! try_or_return {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(e) => return ApiResponse::new(e.error_code, e.message),
        }
    };
}

#[post("admin/network")]
pub async fn create_network(db: Data<Database>, request: Json<SubmitCreateNetwork>) -> ApiResponse {
    let network_type = try_or_return_string!(Network::from_str(request.network_type.clone()));
    let chain_id = request.chain_id.clone();

    let new_network = NetworkManager::new(network_type, chain_id);
    let response_network = try_or_return!(db.create_network(new_network).await);

    ApiResponse::new(201, format!("{:?}", response_network))
}

#[patch("admin/network")]
pub async fn update_network_chain_id(
    db: Data<Database>,
    request: Json<SubmitUpdateNetwork>,
) -> ApiResponse {
    let old_chain_id = request.old_chain_id.clone();
    let new_chain_id = request.new_chain_id.clone();

    let mut network = try_or_return!(db.get_network_via_chain_id(old_chain_id).await);
    network.update_chain_id(new_chain_id);
    let response_network = try_or_return!(db.update_network(network).await);

    ApiResponse::new(200, format!("{:?}", response_network))
}

#[get("admin/networks")]
pub async fn get_all_network(db: Data<Database>) -> ApiResponse {
    let response_network = try_or_return!(db.get_all_networks().await);

    ApiResponse::new(201, format!("{:?}", response_network))
}

#[delete("admin/network")]
pub async fn delete_network(db: Data<Database>, request: Json<SubmitDeleteNetwork>) -> ApiResponse {
    let network_type = request.network_type.clone();

    let network = try_or_return_string!(Network::from_str(network_type));

    let response_network = try_or_return!(db.delete_network(network).await);

    ApiResponse::new(200, format!("{:?}", response_network))
}

#[get("admin/network/{network_type}/last_scanned_block")]
pub async fn get_last_scanned_block(db: Data<Database>, path: Path<String>) -> ApiResponse {
    let network_type = path.into_inner();

    let network = try_or_return_string!(Network::from_str(network_type));

    let response_block_number = try_or_return!(db.get_last_scanned_block(network).await);

    ApiResponse::new(200, format!("{}", response_block_number))
}

#[patch("admin/network/last_scanned_block")]
pub async fn set_last_scanned_block(
    db: Data<Database>,
    request: Json<SubmitSetLastBlock>,
) -> ApiResponse {
    let network_type = request.network_type.clone();
    let block_number: u128 = request.last_scanned_block;

    let network = try_or_return_string!(Network::from_str(network_type));

    let response_network =
        try_or_return!(db.update_last_scanned_block(network, block_number).await);

    ApiResponse::new(200, format!("{:?}", response_network))
}
