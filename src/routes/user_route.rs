use crate::models::network_model::Network;
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
pub struct SubmitCreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct SubmitAddWallet {
    pub user_id: String,
    pub wallet_address: String,
    pub network: String,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct SubmitUpdateWallet {
    pub user_id: String,
    pub wallet_address: String,
    pub new_network: String,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct SubmitDeleteWallet {
    pub user_id: String,
    pub wallet_address: String,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct SubmitGetProfile {
    pub user_id: String,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct SubmitGetProfileViaEmail {
    pub email: String,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct SubmitGetUserViaNetwork {
    pub network: String,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct SubmitUpdateUserEmail {
    old_email: String,
    new_email: String,
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

#[post("/user")]
pub async fn create_user(db: Data<Database>, request: Json<SubmitCreateUser>) -> ApiResponse {
    let user = User::new(request.name.clone(), request.email.clone(), Vec::new()).unwrap();

    match db.create_user(user.clone()).await {
        Ok(_result) => ApiResponse::new(201, format!("{:?}", user.user_uuid.clone())),
        Err(e) => ApiResponse::new(e.error_code, format!("Error creating user: {}", e.message)),
    }
}

#[get("/user")]
pub async fn get_profile(db: Data<Database>, request: Json<SubmitGetProfile>) -> ApiResponse {
    let user: User = try_or_return!(db.get_user_via_id(request.user_id.clone()).await);
    ApiResponse::new(200, format!("{:?}", user))
}

#[get("/users")]
pub async fn get_all_users(db: Data<Database>) -> ApiResponse {
    let users: Vec<User> = try_or_return!(db.get_all_users().await);
    return ApiResponse::new(200, format!("{:?}", users));
}

#[get("/users/{network}")]
pub async fn get_all_users_via_network(
    db: Data<Database>,
    request: Path<SubmitGetUserViaNetwork>,
) -> ApiResponse {
    let network = try_or_return_string!(Network::from_str(request.into_inner().network));

    let users: Vec<User> = try_or_return!(db.get_all_users_via_network(network).await);
    return ApiResponse::new(200, format!("{:?}", users));
}

#[get("/wallets/{network}")]
pub async fn get_all_wallets_via_network(
    db: Data<Database>,
    request: Path<SubmitGetUserViaNetwork>,
) -> ApiResponse {
    let network = try_or_return_string!(Network::from_str(request.into_inner().network));

    let wallets: Vec<Wallet> = try_or_return!(db.get_all_wallets_via_network(network).await);
    return ApiResponse::new(200, format!("{:?}", wallets));
}

#[get("/user/email")]
pub async fn get_user_via_email(
    db: Data<Database>,
    request: Json<SubmitGetProfileViaEmail>,
) -> ApiResponse {
    let email_address = request.email.clone();

    let user: User = try_or_return!(db.get_user_via_email(email_address).await);
    return ApiResponse::new(200, format!("{:?}", user));
}

#[patch("/user/email")]
pub async fn update_user_email(
    db: Data<Database>,
    request: Json<SubmitUpdateUserEmail>,
) -> ApiResponse {
    let email_address = request.old_email.clone();
    let new_email_address = request.new_email.clone();

    let mut user: User = try_or_return!(db.get_user_via_email(email_address).await);
    try_or_return_string!(user.change_email(new_email_address));
    let response_user = try_or_return!(db.update_user(user).await);
    return ApiResponse::new(200, format!("{:?}", response_user));
}

#[post("/user/wallets")]
pub async fn add_wallet(db: Data<Database>, request: Json<SubmitAddWallet>) -> ApiResponse {
    let user_id = request.user_id.clone();
    let user_wallet = try_or_return_string!(Wallet::new(
        request.wallet_address.clone(),
        request.network.clone()
    ));
    let mut user: User = try_or_return!(db.get_user_via_id(user_id.clone()).await);
    try_or_return_string!(user.add_wallet(user_wallet));
    let response_user = try_or_return!(db.update_user(user).await);
    return ApiResponse::new(200, format!("{:?}", response_user));
}

#[patch("/user/wallets")]
pub async fn update_wallets(db: Data<Database>, request: Json<SubmitUpdateWallet>) -> ApiResponse {
    let user_id = request.user_id.clone();
    let wallets_address = request.wallet_address.clone();
    let new_network = request.new_network.clone();

    let mut user: User = try_or_return!(db.get_user_via_id(user_id.clone()).await);
    try_or_return_string!(user.update_wallet_network(
        &wallets_address,
        try_or_return_string!(Network::from_str(new_network)),
    ));
    let response_user = try_or_return!(db.update_user(user).await);
    return ApiResponse::new(200, format!("{:?}", response_user));
}

#[delete("/user/wallets")]
pub async fn delete_wallet(db: Data<Database>, request: Json<SubmitDeleteWallet>) -> ApiResponse {
    let user_id = request.user_id.clone();
    let wallets_address = request.wallet_address.clone();

    let mut user: User = try_or_return!(db.get_user_via_id(user_id.clone()).await);
    try_or_return_string!(user.remove_wallet(&wallets_address));
    let response_user = try_or_return!(db.update_user(user).await);
    return ApiResponse::new(
        200,
        format!(
            "{:?}",
            response_user
                .get_user_wallets()
                .expect("Error getting wallets details")
        ),
    );
}

#[get("/user/wallets")]
pub async fn get_wallets(db: Data<Database>, request: Json<SubmitGetProfile>) -> ApiResponse {
    let user_id = request.user_id.clone();

    let user: User = try_or_return!(db.get_user_via_id(user_id.clone()).await);
    let user_wallets = user.wallets;
    return ApiResponse::new(200, format!("{:?}", user_wallets));
}
