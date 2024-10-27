use crate::models::user_model::{Network, User, Wallet};
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
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubmitCreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct SubmitAddWallet {
    pub user_id: String,
    pub wallets: Vec<Wallet>,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct SubmitUpdateWallet {
    pub user_id: String,
    pub wallets_address: String,
    pub new_network: Network,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct SubmitDeleteWallet {
    pub user_id: String,
    pub wallets_address: String,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct SubmitGetProfile {
    pub user_id: String,
}

#[post("/user")]
pub async fn create_user(request: Json<SubmitCreateUser>) -> ApiResponse {
    let user = User::new(request.name.clone(), request.email.clone(), Vec::new()).unwrap();
    ApiResponse::new(
        201,
        format!("User created, User Id is: {:?}", user.user_uuid),
    )
}

#[patch("/user/wallets/add")]
pub async fn add_wallets(request: Json<SubmitAddWallet>) -> ApiResponse {
    let user_id = request.user_id.clone();
    // let wallets = request.wallets.clone();

    // Implement functionality to fetch user from database by user_id, then add wallets to the user
    ApiResponse::new(
        200,
        format!("User wallets updated, User Id is: {}", user_id),
    )
}

#[patch("/user/wallets/update")]
pub async fn update_wallets(request: Json<SubmitUpdateWallet>) -> ApiResponse {
    let user_id = request.user_id.clone();
    let wallets_address = request.wallets_address.clone();
    let new_network = request.new_network.clone();

    // Implement functionality to fetch user from database by user_id, then call the change network function.

    ApiResponse::new(
        200,
        format!("User wallets updated, User Id is: {}", user_id),
    )
}

#[delete("/user")]
pub async fn delete_wallets(request: Json<SubmitDeleteWallet>) -> ApiResponse {
    let user_id = request.user_id.clone();
    let wallets_address = request.wallets_address.clone();

    // Implement functionality to fetch user from database by user_id, then call the delete wallet function.

    ApiResponse::new(
        200,
        format!("User wallets updated, User Id is: {}", user_id),
    )
}

#[get("/user/{user_id}/wallets")]
pub async fn get_wallets(user_identifier: Path<SubmitGetProfile>) -> ApiResponse {
    let user_id = user_identifier.into_inner().user_id;

    // Implement functionality to fetch user from database by user_id, then call the get wallet function.

    ApiResponse::new(
        200,
        format!("User wallets displayed, User Id is: {}", user_id),
    )
}

#[get("/user/{user_id}")]
pub async fn get_profile(request: Json<SubmitGetProfile>) -> ApiResponse {
    let user_id = request.user_id.clone();

    // Implement functionality to fetch user from database by user_id, then return the users complete profile.

    ApiResponse::new(
        200,
        format!("User profile displayed, User Id is: {}", user_id),
    )
}
