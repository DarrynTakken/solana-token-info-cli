use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::*;
use std::net::IpAddr;
use regex::Regex;
use url::Url;
use serde::{Deserialize, Serialize};

use crate::config::consts::{METAPLEX_PROGRAM_ID};
use crate::errors::errors::AppError;

// Struct to handle token information responses, with optional fields for flexibility
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_entries: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instagram: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply: Option<String>,
}

// Asynchronous function to get detailed token information
pub async fn get_token_info(client: &RpcClient, token_address: &str) -> Result<TokenInfo, Box<dyn std::error::Error>> {
    let token_pubkey = Pubkey::from_str(token_address)
        .map_err(|_| AppError::InvalidTokenAddress(token_address.to_string()))?;

    let metadata_pubkey = get_metadata_address(&token_pubkey);

    // Fetch the metadata account associated with the token
    let metadata_account = client.get_account(&metadata_pubkey)
        .map_err(|e| AppError::GeneralError(format!("Failed to fetch metadata account - {}", e)))?;

    let mut offset = 96; // Set initial offset to skip unnecessary data

    // Read the symbol (10 bytes) from the metadata
    let raw_symbol_data = &metadata_account.data[offset..offset + 10];
    let _symbol = String::from_utf8_lossy(raw_symbol_data)
        .trim_end_matches(char::from(0))
        .to_string();
    offset += 10;

    if raw_symbol_data.contains(&0) {
        offset += raw_symbol_data.iter().filter(|&&x| x == 0).count();
    }

    // Read the URI (200 bytes) where JSON metadata is stored
    let raw_uri_data = &metadata_account.data[offset..offset + 200];
    let uri = String::from_utf8_lossy(raw_uri_data)
        .trim_end_matches(char::from(0))
        .to_string();

    let mut token_info: TokenInfo = get_json_from_uri(&clean_uri(&uri)).await?;

    if let Some(website) = &token_info.website {
        if let Some(clean_domain) = get_cleaned_domain(website) {
            if is_valid_domain(&clean_domain) {
                let dns_entries = get_dns_entries(&clean_domain).await?;
                token_info.dns_entries = Some(dns_entries);
            } else {
                return Err(Box::new(AppError::InvalidDomain(clean_domain)));
            }
        }
    }

    Ok(token_info)
}

// Function to fetch JSON data from a URI and deserialize into TokenInfo
async fn get_json_from_uri(uri: &str) -> Result<TokenInfo, AppError> {
    let response = reqwest::get(uri).await?;
    let token_info: TokenInfo = response.json().await
        .map_err(|_| AppError::DeserializationError("Failed to parse JSON from URI".to_string()))?;
    Ok(token_info)
}

// Function to derive the metadata address using the mint address and Metaplex program ID
fn get_metadata_address(mint: &Pubkey) -> Pubkey {
    let program_id = Pubkey::from_str(METAPLEX_PROGRAM_ID)
        .expect("Failed to parse METAPLEX_PROGRAM_ID");
    let seeds = &[b"metadata", program_id.as_ref(), mint.as_ref()];

    // Find the program address based on the seeds
    let (metadata_pubkey, _) = Pubkey::find_program_address(seeds, &program_id);
    metadata_pubkey
}

// Asynchronous function to perform DNS lookups and return entries (A, AAAA, MX records)
async fn get_dns_entries(domain: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default())?;
    let mut dns_entries = Vec::new();

    if let Ok(lookup_ip) = resolver.lookup_ip(domain).await {
        for ip in lookup_ip {
            dns_entries.push(match ip {
                IpAddr::V4(ipv4) => format!("A Record: IPv4: {}", ipv4),
                IpAddr::V6(ipv6) => format!("AAAA Record: IPv6: {}", ipv6),
            });
        }
    }

    if let Ok(mx_lookup) = resolver.mx_lookup(domain).await {
        for mx in mx_lookup.iter() {
            dns_entries.push(format!("MX Record: Preference: {}, Exchange: {}", mx.preference(), mx.exchange()));
        }
    }

    Ok(dns_entries)
}

// Helper function to validate the domain name using a regular expression
fn is_valid_domain(domain: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    re.is_match(domain)
}

// Helper function to extract and clean the domain from a URL
fn get_cleaned_domain(website: &str) -> Option<String> {
    if let Ok(parsed_url) = Url::parse(website) {
        if let Some(domain) = parsed_url.host_str() {
            return Some(domain.to_string());
        }
    }
    None
}

// Helper function to clean and filter non-printable characters, allowing valid URI characters
fn clean_uri(input: &str) -> String {
    input
        .chars()
        .filter(|&c| c.is_ascii_graphic() || c.is_whitespace() || ['/', ':', '.', '-', '_', '?', '&', '=', '%', '~'].contains(&c))
        .collect()
}
