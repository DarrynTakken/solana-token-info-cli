use clap::Parser;
use bs58;

#[derive(Parser)]
#[command(name = "Solana Token Info CLI")]
#[command(about = "Retrieve Solana token details", long_about = "This CLI tool allows you to retrieve details about a Solana token by providing a token address (Base58 encoded).")]
#[command(author = "Darryn Takken", version = "1.0.0", long_about = None, after_help = "Example:\n  cargo run -- <TOKEN_ADDRESS> (Base58 encoded)")]
pub struct Cli {
    #[arg(required = true, help = "The Solana token address (Base58 encoded)")]
    pub token_address: String,
}

impl Cli {
    pub fn get_validated_token_address(&self) -> Result<String, String> {
        // Check if the provided token address is valid Base58
        if let Ok(_decoded) = bs58::decode(&self.token_address).into_vec() {
            // If decoding is successful, return the original address
            return Ok(self.token_address.clone());
        }

        // If it is not valid Base58, try converting it to Base58
        let encoded = bs58::encode(self.token_address.as_bytes()).into_string();
        Ok(encoded)
    }
}
