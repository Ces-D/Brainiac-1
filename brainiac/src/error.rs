use std::error::Error;

use ollama_rs::error::OllamaError;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BrainiacResponseError {
    pub message: String,
    pub prompt: Option<String>,
}
impl std::fmt::Display for BrainiacResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => {
                write!(f, "{}", json)
            }
            Err(_) => write!(f, "{}", self.message),
        }
    }
}
impl From<OllamaError> for BrainiacResponseError {
    fn from(value: OllamaError) -> Self {
        BrainiacResponseError {
            message: value.to_string(),
            prompt: None,
        }
    }
}

impl Error for BrainiacResponseError {}
