use serde::{Deserialize, Serialize};

#[derive(Serialize, PartialEq, Clone, Deserialize)]
pub struct NetworkManager {
    pub network_type: Network,
    pub chain_id: u16,
    pub last_scanned_block: u128,
}

#[derive(Serialize, PartialEq, Clone, Deserialize, Debug)]
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
