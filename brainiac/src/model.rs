use serde::{Deserialize, Serialize};

#[derive(
    Default,
    strum::Display,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantArray,
    Serialize,
    Deserialize,
    Clone,
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub title: String,
    pub description: String,
    pub author: String,
    pub slug: String,
    pub analytics: AnalyticsMetadata,
    pub interest: InterestMetadata,
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
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

/// The type of output to generate by llm
pub enum ResponseOutputType {
    Title,
    Description,
    Genre,
    Keywords,
}
