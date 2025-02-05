use std::{
    error::Error,
    fmt::{Debug, Display},
};

use ollama_rs::error::OllamaError;

#[derive(Debug)]
pub enum BrainiacError {
    OllamaError(OllamaError),
    /// OllamaError with a message and optional prompt that triggered
    OllamaDetailedError(OllamaError, Option<String>),
    IoError(std::io::Error),
}

impl From<OllamaError> for BrainiacError {
    fn from(value: OllamaError) -> Self {
        BrainiacError::OllamaError(value)
    }
}

impl From<std::io::Error> for BrainiacError {
    fn from(value: std::io::Error) -> Self {
        BrainiacError::IoError(value)
    }
}

impl Display for BrainiacError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrainiacError::OllamaError(ollama_error) => {
                let message = match ollama_error {
                    OllamaError::ToolCallError(tool_call_error) => tool_call_error.to_string(),
                    OllamaError::JsonError(error) => error.to_string(),
                    OllamaError::ReqwestError(error) => error.to_string(),
                    OllamaError::InternalError(internal_ollama_error) => {
                        internal_ollama_error.message.to_owned()
                    }
                    OllamaError::Other(message) => message.to_owned(),
                };
                write!(f, "OllamaError: {}", message)
            }
            BrainiacError::OllamaDetailedError(ollama_error, prompt) => {
                let mut e = format!("OllamaError: {}", ollama_error);
                if let Some(prompt) = prompt {
                    e.push_str(&format!("\nPrompt: {}", prompt));
                }
                write!(f, "{}", e)
            }
            BrainiacError::IoError(error) => {
                write!(f, "IoError: {}", error)
            }
        }
    }
}

impl Error for BrainiacError {}
