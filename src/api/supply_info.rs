use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use serde::{Deserialize};

use crate::config::consts::SOLANA_BASE_API_ENDPOINT;
use crate::errors::errors::AppError;

// Struct to deserialize and handle the response from the get_supply_info function
#[derive(Deserialize)]
pub struct SupplyInfo {
    pub amount: String,
}

// Asynchronous function to retrieve token supply information from Solana
// Takes a token address as a string reference and returns a Result with SupplyInfo or AppError
pub async fn get_supply_info(token_address: &str) -> Result<SupplyInfo, AppError> {
    let client = RpcClient::new(SOLANA_BASE_API_ENDPOINT);
    
    let token_pubkey = Pubkey::from_str(token_address)
        .map_err(|_| AppError::InvalidTokenAddress(token_address.to_string()))?;

    // Fetch the token supply information using the RPC client
    // If the request fails, return an AppError for supply info failure
    let supply_response = client.get_token_supply(&token_pubkey)
        .map_err(|_| AppError::SupplyInfoError("Failed to fetch supply info".to_string()))?;

    let amount = supply_response.amount.parse::<String>()
        .map_err(|_| AppError::SupplyInfoError("Failed to parse supply amount".to_string()))?;

    Ok(SupplyInfo { amount })
}
