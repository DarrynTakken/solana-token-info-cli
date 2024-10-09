mod cli;
mod api;
mod config;
mod errors;

use clap::Parser;
use serde_json;

use crate::cli::args::Cli;
use solana_client::rpc_client::RpcClient;
use crate::api::{get_token_info, get_supply_info};
use crate::config::consts::SOLANA_BASE_API_ENDPOINT;
use crate::errors::errors::AppError;

// Entry point of the asynchronous main function, which is executed using the Tokio runtime
#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Validate or convert the provided token address
    let token_address = match cli.get_encoded_token_address() {
        Ok(address) => address,
        Err(e) => {
            eprintln!("{}", AppError::InvalidTokenAddress(e));
            return;
        }
    };

    let client = RpcClient::new(SOLANA_BASE_API_ENDPOINT);

    // Perform concurrent asynchronous tasks to fetch token info and supply info
    let token_info_task = get_token_info(&client, &token_address);
    let supply_info_task = get_supply_info(&client, &token_address);
    let (token_info_task, supply_info_task) = tokio::join!(token_info_task, supply_info_task);

    // Handle the results of both tasks
    match (token_info_task, supply_info_task) {
        (Ok(mut token_info), Ok(supply_info)) => {
            token_info.supply = Some(supply_info.amount);

            if let Some(desc) = token_info.description.as_mut() {
                *desc = clean_description(desc);
            }

            // Serialize the updated token information into pretty-printed JSON and display it
            match serde_json::to_string_pretty(&token_info) {
                Ok(json) => println!("Token Information:\n{}", json),
                Err(e) => eprintln!("Serialization error: {}", AppError::DeserializationError(e.to_string())), // Handle serialization error
            }
        }
        // Handle cases where one of the tasks fails
        (Err(e), Ok(_)) => eprintln!("{}", e),
        (Ok(_), Err(e)) => eprintln!("{}", e),
        // Handle cases where both tasks fail, printing either one or both errors
        (Err(e1), Err(e2)) => {
            if e1.to_string() == e2.to_string() {
                eprintln!("{}", e1);
            } else {
                eprintln!("{}", e1);
                eprintln!("{}", e2);
            }
        }
    }
}

// Function to clean a description field by removing control characters from the string
fn clean_description(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_control())
        .collect()
}
