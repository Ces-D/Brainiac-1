use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JSONResponse<T> {
    pub response: T,
}

impl From<String> for JSONResponse<String> {
    fn from(response: String) -> Self {
        Self { response }
    }
}
impl From<Vec<String>> for JSONResponse<Vec<String>> {
    fn from(response: Vec<String>) -> Self {
        Self { response }
    }
}
impl From<&str> for JSONResponse<String> {
    fn from(response: &str) -> Self {
        Self {
            response: response.to_string(),
        }
    }
}

impl Display for JSONResponse<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(&self).expect("Failed to serialize JSONResponse");
        write!(f, "{}", json)
    }
}
impl Display for JSONResponse<Vec<String>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(&self).expect("Failed to serialize JSONResponse");
        write!(f, "{}", json)
    }
}
