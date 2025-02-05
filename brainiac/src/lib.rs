use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use error::BrainiacError;
use gray_matter::ParsedEntityStruct;
use model::{AnalyticsMetadata, ArticleGenre, InterestMetadata, Metadata, ResponseOutputType};
use ollama_rs::Ollama;
use slugify_rs::slugify;

pub mod error;
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
fn sanitize_string(input: String) -> String {
    input.replace("\n", "").replace("\r", "").replace("\t", "")
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

pub fn create_output_file_name(slug: &String) -> PathBuf {
    let file_name = format!("{}.md", slug);
    Path::new(&file_name).to_path_buf()
}

pub struct BrainiacAppend {
    pub source_path: PathBuf,
    pub output_dir_path: Option<PathBuf>,
    pub gen_model: Option<String>,
    pub format_model: Option<String>,
    pub author: String,
}
/// Append metadata to a file.
/// This function has the following side effects:
/// - Reads the file at `source_path`
/// - Generates metadata fields based on the model and path content
/// - Writes the file to `output_path` or std out
pub async fn append_metadata(params: BrainiacAppend) -> Result<Metadata, BrainiacError> {
    let article_content = std::fs::read_to_string(params.source_path).unwrap();
    let instance = Ollama::default();
    let mut generator = ollama::generator::OutputGenerator::new(
        &instance,
        params.gen_model.unwrap_or("deepseek-r1:8b".to_string()),
    );
    generator.set_content(article_content.clone());
    let formatter = ollama::formatter::OutputFormatter::new(
        &instance,
        params
            .format_model
            .unwrap_or("deepseek-r1:1.5b".to_string()),
    );

    let title = generator.generate_output(ResponseOutputType::Title).await?;
    log::trace!("Unproccessed Title: {}\n", title.response);
    let title = formatter
        .format_output(sanitize_string(title.response), ResponseOutputType::Title)
        .await?;
    log::info!("Title: {}", title.response);
    let description = generator
        .generate_output(ResponseOutputType::Description)
        .await?;
    log::trace!("Unproccessed Description: {}\n", description.response);
    let description = formatter
        .format_output(
            sanitize_string(description.response),
            ResponseOutputType::Description,
        )
        .await?;
    log::info!("Description: {}", description.response);
    let genre = generator.generate_output(ResponseOutputType::Genre).await?;
    log::trace!("Unproccessed Genre: {}", genre.response);
    let genre = formatter
        .format_output(sanitize_string(genre.response), ResponseOutputType::Genre)
        .await?;
    log::info!("Genre: {}", genre.response);
    let keywords = generator
        .generate_output(ResponseOutputType::Keywords)
        .await?;
    log::trace!("Unproccessed Keywords: {}", keywords.response);
    let keywords = formatter
        .format_output(
            sanitize_string(keywords.response),
            ResponseOutputType::Keywords,
        )
        .await?;
    log::info!("Keywords: {}", keywords.response);

    let analytics = get_analytics_data(&article_content);
    let metadata = Metadata {
        title: title.response.clone(),
        description: description.response,
        author: params.author,
        slug: slugify!(title.response.as_str()),
        analytics: AnalyticsMetadata {
            reading_time_in_minutes: analytics.reading_time_in_minutes,
            length_in_words: analytics.length_in_words,
            ..Default::default()
        },
        interest: InterestMetadata {
            keywords: vec![keywords.response],
            genre: ArticleGenre::from_str(genre.response.as_str()).unwrap_or_default(),
            ..Default::default()
        },
    };
    let rendered_metadata = generate_article_matter(&metadata);
    println!("{}", rendered_metadata);
    if params.output_dir_path.is_some() {
        let output_dir_path = params.output_dir_path.unwrap();
        let output_path = Path::new(&output_dir_path).join(create_output_file_name(&metadata.slug));
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
