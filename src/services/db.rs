use crate::models::network_model::{Network, NetworkManager};
use crate::models::user_model::User;
use dotenv::dotenv;
use futures_util::stream::StreamExt;
use mongodb::bson::from_document;
use mongodb::error::Error;
use mongodb::{
    bson::doc,
    options::IndexOptions,
    results::{InsertOneResult, UpdateResult},
    Client, Collection, IndexModel,
};
use std::env;
use std::result;

pub struct Database {
    users: Collection<User>,
    networks: Collection<NetworkManager>,
}

#[derive(Debug)]
pub struct DatabaseResponse {
    pub error_code: u16,
    pub message: String,
}

impl DatabaseResponse {
    pub fn new(error_code: u16, message: String) -> Self {
        DatabaseResponse {
            error_code,
            message,
        }
    }
}

macro_rules! try_or_return_string {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(e) => return Err(DatabaseResponse::new(500, e.to_string())),
        }
    };
}

impl Database {
    pub async fn init() -> Self {
        dotenv().ok();
        let db_url = env::var("DB_URL").expect("DB_URL must be set");

        let client = Client::with_uri_str(db_url)
            .await
            .expect("failed to connect");
        let db = client.database("Kharon-crawler");

        let users = db.collection("users");
        let networks = db.collection("networks");
        println!("DATABASE CONNECTION SUCCESSFUL!!!!");
        return Database { users, networks };
    }

    pub async fn create_user(&self, user: User) -> Result<InsertOneResult, DatabaseResponse> {
        let existing_users = self.get_user_via_email(user.email.clone()).await;
        if existing_users.is_err() {
            match self.users.insert_one(user).await {
                Ok(result) => Ok(result),
                Err(e) => Err(DatabaseResponse::new(
                    500,
                    format!("Error creating user: {}", e),
                )),
            }
        } else {
            return Err(DatabaseResponse::new(
                500,
                "User already exists".to_string(),
            ));
        }
    }

    pub async fn change_email(
        &self,
        email: String,
        user_id: String,
    ) -> Result<UpdateResult, DatabaseResponse> {
        let result = try_or_return_string!(
            self.users
                .update_one(
                    doc! {"user_uuid": user_id},
                    doc! {"$set": doc! {"email": email}},
                )
                .await
        );
        Ok(result)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, DatabaseResponse> {
        let mut result = self.users.find(doc! {}).await;
        match result {
            Ok(mut cursor) => {
                let mut users: Vec<User> = Vec::new();
                while let Some(result) = cursor.next().await {
                    match result {
                        Ok(user) => {
                            users.push(user);
                        }
                        Err(err) => return Err(DatabaseResponse::new(500, format!("{}", err))),
                    }
                }
                Ok(users)
            }
            Err(err) => Err(DatabaseResponse::new(500, format!("{}", err))),
        }
    }

    pub async fn get_all_users_via_network(
        &self,
        network: Network,
    ) -> Result<Vec<User>, DatabaseResponse> {
        match self.get_all_users().await {
            Ok(users) => {
                let mut filtered_users: Vec<User> = Vec::new();
                for user in users.iter() {
                    for wallet in user.wallets.iter() {
                        if wallet.network == network {
                            filtered_users.push(user.clone());
                        }
                    }
                }
                Ok(filtered_users)
            }
            Err(err) => return Err(err),
        }
    }

    pub async fn get_user_via_email(&self, email: String) -> Result<User, DatabaseResponse> {
        let result = self.users.find(doc! {"email": email}).await;
        match result {
            Ok(mut cursor) => {
                let mut users: Vec<User> = Vec::new();
                while let Some(doc) = cursor.next().await {
                    match doc {
                        Ok(user) => users.push(user),
                        Err(e) => return Err(DatabaseResponse::new(500, format!("{}", e))),
                    }
                }
                let user_result = users.get(0);
                match user_result {
                    Some(user) => Ok(user.clone()),
                    None => Err(DatabaseResponse::new(404, "User not found".to_string())),
                }
            }
            Err(e) => return Err(DatabaseResponse::new(500, format!("{}", e))),
        }
    }

    pub async fn get_user_via_id(&self, id: String) -> Result<User, DatabaseResponse> {
        let result = self.users.find(doc! {"user_uuid": id}).await;
        match result {
            Ok(mut cursor) => {
                let mut users: Vec<User> = Vec::new();
                while let Some(doc) = cursor.next().await {
                    match doc {
                        Ok(user) => users.push(user),
                        Err(e) => return Err(DatabaseResponse::new(500, format!("{}", e))),
                    }
                }

                let user_result = users.get(0);
                match user_result {
                    Some(user) => Ok(user.clone()),
                    None => Err(DatabaseResponse::new(404, "User not found".to_string())),
                }
            }
            Err(e) => return Err(DatabaseResponse::new(500, format!("{}", e))),
        }
    }

    pub async fn update_user(&self, user: User) -> Result<User, DatabaseResponse> {
        let result = self
            .users
            .replace_one(doc! {"user_uuid": user.user_uuid.clone()}, user.clone())
            .await;
        match result {
            Ok(update_result) => {
                if update_result.modified_count == 0 {
                    Err(DatabaseResponse::new(404, "User not found".to_string()))
                } else {
                    Ok(user)
                }
            }
            Err(e) => Err(DatabaseResponse::new(500, format!("{}", e))),
        }
    }

    pub async fn create_network(
        &self,
        network: NetworkManager,
    ) -> Result<InsertOneResult, DatabaseResponse> {
        match self
            .get_network_via_name(network.network_type.clone())
            .await
        {
            Ok(_network) => {
                return Err(DatabaseResponse::new(
                    500,
                    "Network already exists".to_string(),
                ))
            }
            Err(err) => {
                if err.error_code == 404 {
                    let result = try_or_return_string!(self.networks.insert_one(network).await);
                    Ok(result)
                } else {
                    Err(DatabaseResponse::new(
                        500,
                        format!("{} : {:?}", "Error creating network", err),
                    ))
                }
            }
        }
    }

    pub async fn get_network_via_name(
        &self,
        network: Network,
    ) -> Result<NetworkManager, DatabaseResponse> {
        let network_name = try_or_return_string!(network.as_str());
        let result = self
            .networks
            .find_one(doc! {"network_type": network_name})
            .await;
        match result {
            Ok(Some(network)) => Ok(network),
            Ok(None) => Err(DatabaseResponse::new(
                404,
                format!("{}", "network not found",),
            )),
            Err(err) => Err(DatabaseResponse::new(
                500,
                format!("{} : {:?}", "Error Fetching network", err),
            )),
        }
    }

    pub async fn get_network_via_chain_id(
        &self,
        chain_id: String,
    ) -> Result<NetworkManager, DatabaseResponse> {
        let result = self.networks.find_one(doc! {"chain_id": &chain_id}).await;
        match result {
            Ok(Some(network)) => Ok(network),
            Ok(None) => Err(DatabaseResponse::new(
                500,
                format!("{}", "network not found",),
            )),
            Err(err) => Err(DatabaseResponse::new(
                500,
                format!("{}: {:?}", "Error Fetching network", err),
            )),
        }
    }
}
