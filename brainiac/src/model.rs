use serde::{Deserialize, Serialize};

#[derive(
    Default,
    strum::Display,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantArray,
    Serialize,
    Deserialize,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum ArticleGenre {
    News,
    Technology,
    Health,
    Sports,
    Entertainment,
    Business,
    Science,
    Education,
    Lifestyle,
    Travel,
    Food,
    Politics,
    #[default]
    Opinion,
    History,
    Art,
}

#[derive(Default, strum::Display, strum::EnumString, Clone)]
pub enum SupportedModel {
    /// Llama 3 instruction-tuned models are fine-tuned and optimized for dialogue/chat use cases and outperform many of the available open-source chat models on common benchmarks.
    #[strum(serialize = "llama3")]
    Llama3,
    /// The Meta Llama 3.2 collection of multilingual large language models (LLMs) is a collection of pretrained and instruction-tuned generative models in 1B and 3B sizes (text in/text out). The Llama 3.2 instruction-tuned text only models are optimized for multilingual dialogue use cases, including agentic retrieval and summarization tasks. They outperform many of the available open source and closed chat models on common industry benchmarks.
    #[strum(serialize = "llama3.2:latest")]
    #[default]
    Llama32,
    /// The Meta Llama 3.3 multilingual large language model (LLM) is a pretrained and instruction tuned generative model in 70B (text in/text out). The Llama 3.3 instruction tuned text only model is optimized for multilingual dialogue use cases and outperform many of the available open source and closed chat models on common industry benchmarks.
    #[strum(serialize = "llama3.3")]
    Llama33,

    #[strum(serialize = "qwen2")]
    Qwen2,
    /// Qwen2.5 is the latest series of Qwen large language models. For Qwen2.5, a range of base language models and instruction-tuned models are released, with sizes ranging from 0.5 to 72 billion parameters. Qwen2.5 introduces the following improvements over Qwen2:
    #[strum(serialize = "qwen2.5")]
    Qwen25,
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub description: String,
    pub author: String,
    pub slug: String,
    pub analytics: AnalyticsMetadata,
    pub interest: InterestMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct AnalyticsMetadata {
    pub created_at: chrono::NaiveDate,
    pub length_in_words: u64,
    pub reading_time_in_minutes: u64,
}
impl Default for AnalyticsMetadata {
    fn default() -> Self {
        Self {
            created_at: chrono::Utc::now().date_naive(),
            length_in_words: 0,
            reading_time_in_minutes: 5,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct InterestMetadata {
    pub keywords: Vec<String>,
    pub genre: ArticleGenre,
    pub related_articles: Vec<String>,
}
impl Default for InterestMetadata {
    fn default() -> Self {
        Self {
            keywords: vec![],
            genre: ArticleGenre::Opinion,
            related_articles: vec![],
        }
    }
}
