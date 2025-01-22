use std::fmt::Display;

use crate::model::SupportedModel;

const LINE_SEPARATOR: &str = "\n";
const EXAMPLE_SEPARATOR: &str = "\n~~~\n";

pub struct Prompt {
    lines: Vec<String>,
    max_content_length: usize,
}

impl Prompt {
    pub fn new(model: SupportedModel) -> Prompt {
        let max_content_length = max_content_length(model);
        Prompt {
            lines: Vec::new(),
            max_content_length,
        }
    }

    /// Push a new line to the prompt. This can be a prefix, suffix, or other. Handles line
    /// separation
    pub fn push(&mut self, line: &str) {
        self.lines.push(line.to_string());
        self.lines.push(LINE_SEPARATOR.to_string());
    }

    /// Push a new example to the prompt. Separates the example from the rest of the prompt
    /// using custom syntax
    pub fn push_example(&mut self, example: String) {
        if self.lines.last() == Some(&EXAMPLE_SEPARATOR.to_string()) {
            self.lines.pop();
        } else {
            self.lines.push(EXAMPLE_SEPARATOR.to_string());
        }
        self.lines.push(example);
        self.lines.push(EXAMPLE_SEPARATOR.to_string());
    }
}

impl Display for Prompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prompt = self.lines.join("");
        if prompt.len() > self.max_content_length {
            return Err(std::fmt::Error);
        }
        write!(f, "{}", prompt)
    }
}

/// see - https://github.com/meta-llama/llama-models/tree/main/models
/// see - https://huggingface.co/Qwen/Qwen2.5-72B
fn max_content_length(model: SupportedModel) -> usize {
    match model {
        SupportedModel::Llama3 => 8_000,
        SupportedModel::Llama32 => 128_000,
        SupportedModel::Llama33 => 128_000,
        SupportedModel::Qwen2 => 131_000,
        SupportedModel::Qwen25 => 131_000,
    }
}
