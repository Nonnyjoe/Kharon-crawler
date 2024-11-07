use super::network_model::Network;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
pub struct Wallet {
    pub wallet_address: String,
    pub network: Network,
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
