use reqwest;
use serde_json;
use bs58;
use crate::models::TokenInfo;
use crate::config::SOLANA_BASE_API_ENDPOINT;

pub async fn get_token_info(token_address: &str) -> Result<TokenInfo, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let params = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getAccountInfo",
        "params": [
            token_address,
            {
                "encoding": "base58"
            }
        ],
    });

    let response = client
        .post(SOLANA_BASE_API_ENDPOINT)
        .json(&params)
        .send()
        .await?
        .text()
        .await?;

    let rpc_response: serde_json::Value = serde_json::from_str(&response)?;

    // Check if there was an error in the response
    if let Some(error) = rpc_response.get("error") {
        let error_message = error
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("Unknown error");
        return Err(error_message.into());
    }

    // Parse the response if it is successful
    if let Some(result) = rpc_response.get("result") {
        if let Some(value) = result.get("value") {
            if let Some(data_array) = value.get("data").and_then(|d| d.as_array()) {
                if let Some(data) = data_array.get(0).and_then(|d| d.as_str()) {
                    // Decode the Base58 encoded data
                    let decoded_data = bs58::decode(data).into_vec()?;

                    // You will need to understand the structure of the decoded data to extract
                    // the name and symbol. Assuming the data contains plain text:
                    // (this is a simplistic interpretation, real structure may vary)

                    // Convert the decoded bytes to a string (assuming UTF-8 encoding)
                    if let Ok(decoded_str) = String::from_utf8(decoded_data) {
                        // Extract the token name and symbol from the decoded string.
                        // You need to adjust this parsing based on the actual encoding format.
                        let name = "ExampleTokenName"; // Placeholder
                        let symbol = "SYM"; // Placeholder

                        return Ok(TokenInfo {
                            name: name.to_string(),
                            symbol: symbol.to_string(),
                        });
                    }
                }
            }
        }
    }

    Err("Unable to parse token information".into())
}
