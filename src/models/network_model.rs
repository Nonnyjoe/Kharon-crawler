use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, PartialEq, Clone, Deserialize)]
pub struct NetworkManager {
    pub network_type: Network,
    pub chain_id: String,
    #[serde(with = "serde_with::rust::display_fromstr")]
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

impl NetworkManager {
    pub fn new(network_type: Network, chain_id: String) -> Self {
        NetworkManager {
            network_type,
            chain_id,
            last_scanned_block: 0,
        }
    }

    pub fn update_last_scanned_block(&mut self, block_number: u128) {
        self.last_scanned_block = block_number;
    }

    pub fn get_network_type(&self) -> Network {
        self.network_type.clone()
    }

    pub fn get_chain_id(&self) -> String {
        self.chain_id.to_string()
    }

    pub fn get_last_scanned_block(&self) -> u128 {
        self.last_scanned_block
    }

    pub fn update_chain_id(&mut self, chain_id: String) {
        self.chain_id = chain_id;
    }
}
