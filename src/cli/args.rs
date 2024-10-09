use clap::Parser;
use bs58;

// Define the CLI structure and metadata using Clap's `Parser`
#[derive(Parser)]
#[command(
    name = "Solana Token Info CLI",
    author = "Darryn Takken",
    version = "1.0.0",
    about = "Retrieve Solana token details",
    long_about = "This CLI tool allows you to retrieve details about a Solana token by providing a token address.",
    after_help = "Example:\n  cargo run -- <TOKEN_ADDRESS>\n\nYou need to provide a valid Solana token address as an argument to retrieve its details."
)]
pub struct Cli {
    #[arg(
        required = true,
        help = "The Solana token address to retrieve information for."
    )]
    pub token_address: String,
}

impl Cli {
    // Method to return a valid Base58 encoded token address
    pub fn get_encoded_token_address(&self) -> Result<String, String> {
        // Try decoding the provided token address as Base58
        if let Ok(_decoded) = bs58::decode(&self.token_address).into_vec() {
            return Ok(self.token_address.clone());
        }

        // If it's not a valid Base58, encode it as Base58 and return the encoded result
        let encoded = bs58::encode(self.token_address.as_bytes()).into_string();
        Ok(encoded)
    }
}
