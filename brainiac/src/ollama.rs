use std::str::FromStr;

use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use strum::IntoEnumIterator;

use crate::model::{ArticleGenre, SupportedModel};
// TODO: fix the prompts to only return the desired format

pub async fn model_is_available(instance: &Ollama, model_name: &SupportedModel) -> bool {
    let local_models = instance.list_local_models().await.unwrap();
    local_models
        .into_iter()
        .any(|model| model.name == model_name.to_string())
}

pub async fn generate_article_summary(
    instance: &Ollama,
    model_name: &SupportedModel,
    article: &String,
) -> ollama_rs::error::Result<String> {
    let prompt = format!(
        "Summarize this article for a general audience in four sentences. {}",
        format_article(article)
    );
    let generation = instance
        .generate(GenerationRequest::new(model_name.to_string(), prompt))
        .await?;
    Ok(generation.response)
}

pub async fn generate_article_title(
    instance: &Ollama,
    model_name: &SupportedModel,
    article: &String,
) -> ollama_rs::error::Result<String> {
    let prompt = format!(
        "Generate a title for this article that summarizes the main point. {}",
        format_article(article)
    );
    let generation = instance
        .generate(GenerationRequest::new(model_name.to_string(), prompt))
        .await?;
    Ok(generation.response)
}

pub async fn generate_article_keywords(
    instance: &Ollama,
    model_name: &SupportedModel,
    article: &String,
) -> ollama_rs::error::Result<Vec<String>> {
    let format_keywords = "OUTPUT FORMAT: 6 keywords, separated by commas\n";
    let prompt = format!(
        "You are an expert in  SEO and keyword research. What are the 6 keywords that summarize the main purpose of this article. {}{}",
       format_keywords, 
       format_article(article)
    );
    let generation = instance
        .generate(GenerationRequest::new(model_name.to_string(), prompt))
        .await?;
    Ok(generation.response.split(',').map(|s| s.trim().to_string()).collect())
}

pub async fn generate_article_genre(
    instance: &Ollama,
    model_name: &SupportedModel,
    article: &String,
) -> ollama_rs::error::Result<ArticleGenre> {
    let prompt = format!(
        "You are a literary critic. What genre does this article belong to? Select a genre from the following list. {} {}",
        format_genres(),
        format_article(article)
    );
    let generation = instance
        .generate(GenerationRequest::new(model_name.to_string(), prompt))
        .await?;
    Ok(
        ArticleGenre::from_str(generation.response.to_uppercase().as_str())
            .unwrap_or(ArticleGenre::default()),
    )
}

fn format_article(article: &String) -> String {
    format!("ARTICLE: {}\n", article)
}
fn format_genres() -> String {
    let genres: Vec<String> = ArticleGenre::iter()
        .map(|genre| genre.to_string())
        .collect();
    format!("GENRES: {}\n", genres.join(","))
}
