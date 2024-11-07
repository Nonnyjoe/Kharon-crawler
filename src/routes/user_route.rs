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

// #[derive(Serialize, Clone, Deserialize)]
// pub struct SubmitAddWallet {
//     pub user_id: String,
//     pub wallets: Vec<Wallet>,
// }

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
        Ok(_result) => ApiResponse::new(
            201,
            format!("User created, User Id is: {:?}", user.user_uuid.clone()),
        ),
        Err(e) => ApiResponse::new(e.error_code, format!("Error creating user: {}", e.message)),
    }
}

#[post("/user/wallets")]
pub async fn add_wallet(db: Data<Database>, request: Json<SubmitAddWallet>) -> ApiResponse {
    let user_id = request.user_id.clone();
    let user_wallet = try_or_return_string!(Wallet::new(
        request.wallet_address.clone(),
        request.network.clone()
    ));
    let mut user: User = try_or_return!(db.get_user_via_id(user_id.clone()).await);
    if user.wallets.contains(&user_wallet) {
        return ApiResponse::new(
            400,
            "Wallet already exists in the user's wallets".to_string(),
        );
    } else {
        user.add_wallet(user_wallet).unwrap();
        let response_user = try_or_return!(db.update_user(user).await);
        return ApiResponse::new(
            200,
            format!("User wallet added, User details is: {:?}", response_user),
        );
    }
}

#[patch("/user/wallets")]
pub async fn update_wallets(db: Data<Database>, request: Json<SubmitUpdateWallet>) -> ApiResponse {
    let user_id = request.user_id.clone();
    let wallets_address = request.wallet_address.clone();
    let new_network = request.new_network.clone();

    let mut user: User = try_or_return!(db.get_user_via_id(user_id.clone()).await);
    for wallet in &mut user.wallets {
        if wallet.wallet_address.to_lowercase() == wallets_address.to_lowercase() {
            wallet.network = try_or_return_string!(Network::from_str(new_network));
            let response_user = try_or_return!(db.update_user(user.clone()).await);
            return ApiResponse::new(
                200,
                format!(
                    "User wallets updated, User details are: {:?}",
                    response_user
                ),
            );
        }
    }

    return ApiResponse::new(404, "Wallet not found in the user's wallets".to_string());
}
#[delete("/user/wallets")]
pub async fn delete_wallet(db: Data<Database>, request: Json<SubmitDeleteWallet>) -> ApiResponse {
    let user_id = request.user_id.clone();
    let wallets_address = request.wallet_address.clone();

    let mut user: User = try_or_return!(db.get_user_via_id(user_id.clone()).await);
    if let Some(index) = user
        .wallets
        .iter()
        .position(|w| w.wallet_address.to_lowercase() == wallets_address.to_lowercase())
    {
        user.wallets.remove(index);
        let response_user = try_or_return!(db.update_user(user.clone()).await);
        return ApiResponse::new(
            200,
            format!(
                "Specified wallet deleted, User's updated details are: {:?}",
                response_user
            ),
        );
    }

    return ApiResponse::new(404, "Wallet not found in the user's wallets".to_string());
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

#[get("/user/user_id")]
pub async fn get_profile(request: Json<SubmitGetProfile>) -> ApiResponse {
    let user_id = request.user_id.clone();

    // Implement functionality to fetch user from database by user_id, then return the users complete profile.

    ApiResponse::new(
        200,
        format!("User profile displayed, User Id is: {}", user_id),
    )
}

#[get("/users")]
pub async fn get_all_users(db: Data<Database>) -> ApiResponse {
    let network = Network::Starknet;

    match db.get_all_users(network).await {
        Ok(users) => ApiResponse::new(
            200,
            format!("All users retrieved, Total Users: {}", users.len()),
        ),
        Err(e) => ApiResponse::new(
            e.error_code,
            format!("Error retrieving users: {}", e.message),
        ),
    }
}

#[get("/user/{email}")]
pub async fn get_user_via_email(db: Data<Database>, request: Path<String>) -> ApiResponse {
    let email_address = request.into_inner().clone();
    println!("Fetching user with email: {}", email_address);

    match db.get_user_via_email(email_address).await {
        Ok(users) => ApiResponse::new(
            200,
            format!("User retrieved by email, Users details are: {:?}", users),
        ),
        Err(e) => ApiResponse::new(
            e.error_code,
            format!("Error retrieving users: {}", e.message),
        ),
    }
}
