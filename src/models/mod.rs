use serde::Deserialize;

#[derive(Deserialize)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
}