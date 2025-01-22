use std::str::FromStr;

use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

use crate::error::BrainiacResponseError;
use crate::factory::response::JSONResponse;
use crate::factory::templates::{
    AbstractTemplate, GenreTemplate, KeywordTemplate, PromptTemplate, TitleTemplate,
};
use crate::model::{ArticleGenre, SupportedModel};

pub async fn model_is_available(instance: &Ollama, model_name: &SupportedModel) -> bool {
    let local_models = instance.list_local_models().await.unwrap();
    local_models
        .into_iter()
        .any(|model| model.name == model_name.to_string())
}

pub async fn generate_article_summary(
    instance: &Ollama,
    model_name: SupportedModel,
    article: &str,
) -> Result<String, BrainiacResponseError> {
    let template = AbstractTemplate::new(article, model_name.clone()).include_examples();
    let generation = instance
        .generate(GenerationRequest::new(
            model_name.to_string(),
            template.as_prompt_string(),
        ))
        .await?;
    match serde_json::from_str::<JSONResponse<String>>(generation.response.as_str()) {
        Ok(output) => Ok(output.response),
        Err(error) => Err(BrainiacResponseError {
            message: error.to_string(),
            prompt: Some(template.as_prompt_string()),
        }),
    }
}

pub async fn generate_article_title(
    instance: &Ollama,
    model_name: SupportedModel,
    content: &str,
) -> Result<String, BrainiacResponseError> {
    let template = TitleTemplate::new(content, model_name.clone()).include_examples();
    let generation = instance
        .generate(GenerationRequest::new(
            model_name.to_string(),
            template.as_prompt_string(),
        ))
        .await?;
    match serde_json::from_str::<JSONResponse<String>>(generation.response.as_str()) {
        Ok(output) => Ok(output.response),
        Err(error) => Err(BrainiacResponseError {
            message: error.to_string(),
            prompt: Some(template.as_prompt_string()),
        }),
    }
}

pub async fn generate_article_keywords(
    instance: &Ollama,
    model_name: SupportedModel,
    article: &str,
) -> Result<Vec<String>, BrainiacResponseError> {
    let template = KeywordTemplate::new(article, model_name.clone()).include_examples();
    let generation = instance
        .generate(GenerationRequest::new(
            model_name.to_string(),
            template.as_prompt_string(),
        ))
        .await?;
    match serde_json::from_str::<JSONResponse<Vec<String>>>(generation.response.as_str()) {
        Ok(output) => Ok(output.response),
        Err(error) => Err(BrainiacResponseError {
            message: error.to_string(),
            prompt: Some(template.as_prompt_string()),
        }),
    }
}

pub async fn generate_article_genre(
    instance: &Ollama,
    model_name: SupportedModel,
    article: &str,
) -> Result<ArticleGenre, BrainiacResponseError> {
    let template = GenreTemplate::new(article, model_name.clone());
    let generation = instance
        .generate(GenerationRequest::new(
            model_name.to_string(),
            template.as_prompt_string(),
        ))
        .await?;
    match serde_json::from_str::<JSONResponse<String>>(generation.response.as_str()) {
        Ok(output) => match ArticleGenre::from_str(output.response.as_str()) {
            Ok(genre) => Ok(genre),
            Err(e) => Err(BrainiacResponseError {
                message: e.to_string(),
                prompt: Some(template.as_prompt_string()),
            }),
        },
        Err(error) => Err(BrainiacResponseError {
            message: error.to_string(),
            prompt: Some(template.as_prompt_string()),
        }),
    }
}
