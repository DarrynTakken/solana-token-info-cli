mod cli;
mod solana;
mod models;
mod config;

use crate::cli::args::Cli;
use crate::solana::get_token_info;
use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Validate or convert the token address
    let token_address = match cli.get_validated_token_address() {
        Ok(address) => address,
        Err(e) => {
            eprintln!("Invalid token address: {}", e);
            return;
        }
    };

    // Retrieve token data
    match get_token_info(&token_address).await {
        Ok(token_info) => {
            println!("Token Name: {}", token_info.name);
            println!("Symbol: {}", token_info.symbol);
        }
        Err(e) => eprintln!("Failed to retrieve token information: {}", e),
    }
}
