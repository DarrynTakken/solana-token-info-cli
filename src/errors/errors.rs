use thiserror::Error;

// Define the custom error type `AppError` using the `Error` derived from `thiserror`
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid token address - {0}")]
    InvalidTokenAddress(String),

    #[error("Invalid domain - {0}")]
    InvalidDomain(String),

    #[error("Deserialization error - {0}")]
    DeserializationError(String),

    #[error("Solana client error - {0}")]
    SolanaClientError(#[from] solana_client::client_error::ClientError),

    #[error("Request error - {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("{0}")]
    GeneralError(String),

    // #[error("Failed to fetch supply info - {0}")]
    // SupplyInfoError(String),

    // #[error("Failed to fetch token info - {0}")]
    // TokenInfoError(String),

    // #[error("Network request failed: {0}")]
    // NetworkError(String),

    // #[error("Failed to resolve DNS: {0}")]
    // DnsError(String),

    // #[error("Unknown error occurred.")]
    // Unknown,
}
