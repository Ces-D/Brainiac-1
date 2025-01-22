use std::io::Write;
use std::path::{Path, PathBuf};

use gray_matter::ParsedEntityStruct;
use model::{AnalyticsMetadata, InterestMetadata, Metadata};
use ollama::{
    generate_article_genre, generate_article_keywords, generate_article_summary,
    generate_article_title, model_is_available,
};
use slugify_rs::slugify;

pub mod error;
mod factory;
pub mod model;
mod ollama;

fn generate_article_matter(metadata: &Metadata) -> String {
    let matter = gray_matter::Matter::<gray_matter::engine::TOML>::new();
    let t = toml::to_string_pretty(metadata).unwrap();
    format!("{}\n{}\n{}\n", matter.delimiter, t, matter.delimiter)
}
fn parse_article_matter(content: &str) -> Option<gray_matter::ParsedEntityStruct<Metadata>> {
    let matter = gray_matter::Matter::<gray_matter::engine::TOML>::new();
    matter.parse_with_struct::<Metadata>(content)
}

struct AnalyticsData {
    reading_time_in_minutes: u64,
    length_in_words: u64,
}
fn get_analytics_data(content: &str) -> AnalyticsData {
    let reading_options = estimated_read_time::Options::default();
    let reading_time = estimated_read_time::text(content, &reading_options);
    AnalyticsData {
        reading_time_in_minutes: reading_time.seconds() / 60,
        length_in_words: reading_time.word_count(),
    }
}
fn create_ollama_instance() -> ollama_rs::Ollama {
    ollama_rs::Ollama::default()
}
pub fn create_output_file_name(slug: &String) -> PathBuf {
    let file_name = format!("{}.md", slug);
    Path::new(&file_name).to_path_buf()
}

pub struct BrainiacAppend {
    pub source_path: PathBuf,
    pub output_dir_path: Option<PathBuf>,
    pub model: Option<model::SupportedModel>,
    pub author: String,
}
/// Append metadata to a file.
/// This function has the following side effects:
/// - Reads the file at `source_path`
/// - Generates metadata fields based on the model and path content
/// - Writes the file to `output_path` or std out
pub async fn append_metadata(params: BrainiacAppend) -> std::io::Result<Metadata> {
    let article_content = std::fs::read_to_string(params.source_path).unwrap();
    let instance = create_ollama_instance();
    let model = params.model.unwrap_or_default();
    if model_is_available(&instance, &model).await {
        let summary = generate_article_summary(&instance, model.clone(), &article_content)
            .await
            .unwrap();
        let title = generate_article_title(&instance, model.clone(), &summary)
            .await
            .unwrap();
        let keywords = generate_article_keywords(&instance, model.clone(), &article_content)
            .await
            .unwrap();
        let genre = generate_article_genre(&instance, model, &article_content)
            .await
            .unwrap();
        let analytics = get_analytics_data(&article_content);
        let metadata = Metadata {
            title: title.clone(),
            description: summary,
            author: params.author,
            slug: slugify!(title.as_str()),
            analytics: AnalyticsMetadata {
                reading_time_in_minutes: analytics.reading_time_in_minutes,
                length_in_words: analytics.length_in_words,
                ..Default::default()
            },
            interest: InterestMetadata {
                keywords,
                genre,
                ..Default::default()
            },
        };
        let rendered_metadata = generate_article_matter(&metadata);
        println!("{}", rendered_metadata);
        if params.output_dir_path.is_some() {
            let output_dir_path = params.output_dir_path.unwrap();
            let output_path =
                Path::new(&output_dir_path).join(create_output_file_name(&metadata.slug));
            let mut file = std::fs::File::create_new(output_path).unwrap();
            let buffered_content = format!("{}\n{}", rendered_metadata, article_content);
            let _ = file.write(buffered_content.as_bytes())?;
            Ok(metadata)
        } else {
            let file_name = create_output_file_name(&metadata.slug);
            let mut file = std::fs::File::create_new(Path::new(&file_name)).unwrap();
            let buffered_content = format!("{}\n\n{}", rendered_metadata, article_content);
            let _ = file.write(buffered_content.as_bytes())?;
            Ok(metadata)
        }
    } else {
        panic!("Model not available: {}", model);
    }
}

pub struct BrainiacParse {
    pub source_path: String,
}
/// Parse metadata from a file.
/// This function has the following side effects:
/// - Reads the file at `source_path`
/// - Extracts metadata fields from the file
/// - Extracts content from the file
/// - Returns the metadata and content
pub fn parse_metadata(params: BrainiacParse) -> Option<ParsedEntityStruct<model::Metadata>> {
    let content = std::fs::read_to_string(params.source_path).unwrap();
    parse_article_matter(content.as_str())
}
