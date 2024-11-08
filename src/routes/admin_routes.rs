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
