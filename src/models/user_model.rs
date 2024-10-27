use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    pub user_uuid: String,
    pub name: String,
    pub email: String,
    pub wallets: Vec<Wallet>,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct Wallet {
    pub wallet_address: String,
    pub network: Network,
}

#[derive(Serialize, PartialEq, Clone, Deserialize)]
pub enum Network {
    Ethereum,
    Starknet,
    Base,
    Optimism,
}

impl Network {
    pub fn as_str(&self) -> Result<String, String> {
        match self {
            Network::Ethereum => Ok("Ethereum".to_string()),
            Network::Starknet => Ok("Starknet".to_string()),
            Network::Base => Ok("Base".to_string()),
            Network::Optimism => Ok("Optimism".to_string()),
        }
    }

    pub fn from_str(network: String) -> Result<Self, String> {
        let network = match network.to_lowercase().as_str() {
            "ethereum" => Network::Ethereum,
            "starknet" => Network::Starknet,
            "base" => Network::Base,
            "optimism" => Network::Optimism,
            _ => return Err("Invalid network type".to_string()),
        };
        Ok(network)
    }
}

impl Wallet {
    pub fn new(wallet_address: String, network: String) -> Result<Self, String> {
        let network = Network::from_str(network);
        if let Ok(user_network) = network {
            Ok(Wallet {
                wallet_address,
                network: user_network,
            })
        } else {
            Err("Invalid wallet network type".to_string())
        }
    }

    pub fn change_network(&mut self, network: String) -> Result<String, String> {
        let network = Network::from_str(network);
        if let Ok(new_network) = network {
            self.network = new_network;
            Ok("Network updated successfully".to_string())
        } else {
            return Err("Invalid network type".to_string());
        }
    }
}

impl User {
    pub fn new(name: String, email: String, wallets: Vec<Wallet>) -> Result<Self, String> {
        Ok(User {
            user_uuid: Uuid::new_v4().to_string(),
            name,
            email,
            wallets,
        })
    }

    pub fn add_wallet(
        &mut self,
        wallet_address: String,
        wallet_network: String,
    ) -> Result<String, String> {
        let user_wallet = Wallet::new(wallet_address, wallet_network);
        if let Ok(wallet) = user_wallet {
            self.wallets.push(wallet);
            return Ok("Wallet added successfully".to_string());
        } else {
            return Err("Invalid wallet network type".to_string());
        }
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
