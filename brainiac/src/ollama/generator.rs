use crate::model::{ArticleGenre, ResponseOutputType};
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::completion::GenerationResponse;
use ollama_rs::Ollama;
use strum::VariantArray;

pub struct OutputGenerator<'a> {
    instance: &'a Ollama,
    content: Option<String>,
    model: String,
}

impl<'a> OutputGenerator<'a> {
    pub fn new(instance: &'a Ollama, model: String) -> OutputGenerator<'a> {
        OutputGenerator {
            instance,
            model,
            content: None,
        }
    }

    /// Content: The text that will be referenced in the prompt
    pub fn set_content(&mut self, content: String) {
        self.content = Some(content);
    }

    pub async fn generate_output(
        &self,
        output: ResponseOutputType,
    ) -> ollama_rs::error::Result<GenerationResponse> {
        let system = "You are an editor at major publishing company. The following article has just arrived at your desk.";
        let guidelines = match output {
            ResponseOutputType::Title => "What should be the title of this article?",
            ResponseOutputType::Description => "Provide a brief summary of this article.",
            ResponseOutputType::Genre => "What genre does this article belong to?",
            ResponseOutputType::Keywords => "What are some keywords that describe this article?",
        };
        let limitations = match output {
            ResponseOutputType::Title => "The title should be at most 10 words.".to_string(),
            ResponseOutputType::Description => {
                "The summary should be less than 5 sentences in length and be written in a single paragraph.".to_string()
            }
            ResponseOutputType::Genre => {
                format!(
                    "The genre should be a single word and be one of these available options: {}",
                    ArticleGenre::VARIANTS
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            ResponseOutputType::Keywords => {
                "The keywords should be a comma separated list.".to_string()
            }
        };

        let request = GenerationRequest::new(
            self.model.clone(),
            format!("{}\n{}", guidelines, limitations),
        )
        .system(format!(
            "{}\n###Article:\n{}",
            system,
            self.content.clone().expect("Content not set")
        ));

        self.instance.generate(request).await
    }
}
