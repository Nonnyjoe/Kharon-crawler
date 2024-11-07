use super::network_model::Network;
use super::wallet_model::Wallet;
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    pub user_uuid: String,
    pub name: String,
    pub email: String,
    pub wallets: Vec<Wallet>,
}

impl User {
    pub fn new(name: String, email: String, wallets: Vec<Wallet>) -> Result<Self, String> {
        Ok(User {
            user_uuid: Uuid::new_v4().to_string(),
            name,
            email: email.to_lowercase(),
            wallets,
        })
    }

    pub fn add_wallet(&mut self, user_wallet: Wallet) -> Result<User, String> {
        self.wallets.push(user_wallet);
        Ok(self.clone())
    }

    pub fn get_wallet_by_address(&self, address: &str) -> Result<&Wallet, String> {
        let wallet = self.wallets.iter().find(|w| w.wallet_address == address);
        match wallet {
            Some(w) => Ok(w),
            None => Err("Could not find wallet with given address".to_string()),
        }
    }

    pub fn remove_wallet(&mut self, address: &str) -> Result<String, String> {
        let index = self
            .wallets
            .iter()
            .position(|w| w.wallet_address == address);
        match index {
            Some(i) => {
                self.wallets.remove(i);
                Ok("Wallet removed successfully".to_string())
            }
            None => Err("Could not find wallet with given address".to_string()),
        }
    }

    pub fn update_wallet_network(
        &mut self,
        address: &str,
        new_network: Network,
    ) -> Result<String, String> {
        let user_wallet = self
            .wallets
            .iter_mut()
            .find(|w| w.wallet_address == address);

        match user_wallet {
            Some(wallet) => {
                wallet.network = new_network;
                return Ok("Wallet network updated successfully".to_string());
            }
            None => Err("Could not find wallet with given address".to_string()),
        }
    }

    pub fn get_user_wallets(&self) -> Result<Vec<Wallet>, String> {
        return Ok(self.wallets.clone());
    }

    pub fn change_email(&mut self, new_email: String) -> Result<String, String> {
        self.email = new_email;
        return Ok("Email updated successfully".to_string());
    }
}
