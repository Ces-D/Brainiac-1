use super::prompt::Prompt;
use crate::factory::constants::{
    select_examples, KEYWORD_EXAMPLES, SUMMARIZATION_EXAMPLES, TITLE_EXAMPLES,
};
use crate::factory::response::JSONResponse;
use crate::model::{ArticleGenre, SupportedModel};
use strum::IntoEnumIterator;

/// A trait for generating a prompt string
pub trait PromptTemplate {
    fn as_prompt_string(&self) -> String;
}

macro_rules! impl_prompt_template {
    ($type:ty) => {
        impl PromptTemplate for $type {
            fn as_prompt_string(&self) -> String {
                self.prompt.to_string()
            }
        }
    };
}

/// A template for generating a summary prompt for an article or blog post
pub struct AbstractTemplate {
    prompt: Prompt,
}
// FIXME: This continues to return a json parse error
impl AbstractTemplate {
    pub fn new(content: &str, model: SupportedModel) -> Self {
        let mut prompt = Prompt::new(model);
        prompt.push(content);
        prompt.push("Analyze the text above and generate an abstract that captures the main points of the content. The abstract should be less than 5 sentences in length.");

        Self { prompt }
    }

    pub fn include_examples(mut self) -> Self {
        self.prompt.push("Examples of good responses are:");
        let selections = select_examples(SUMMARIZATION_EXAMPLES.to_vec(), 3);

        for selection in selections {
            self.prompt
                .push_example(JSONResponse::from(selection).to_string());
        }

        self
    }
}
impl_prompt_template!(AbstractTemplate);

pub struct TitleTemplate {
    prompt: Prompt,
}
impl TitleTemplate {
    pub fn new(content: &str, model: SupportedModel) -> Self {
        let mut prompt = Prompt::new(model);
        prompt.push(content);
        prompt.push("Analyze the summary above and generate an interesting title that capture the main purpose of the article.The title should be short and less than a sentence in length.");

        Self { prompt }
    }

    pub fn include_examples(mut self) -> Self {
        self.prompt.push("Examples of good responses are:");
        let selections = select_examples(TITLE_EXAMPLES.to_vec(), 3);

        for selection in selections {
            self.prompt
                .push_example(JSONResponse::from(selection).to_string());
        }

        self
    }
}

impl_prompt_template!(TitleTemplate);

pub struct KeywordTemplate {
    prompt: Prompt,
}
impl KeywordTemplate {
    pub fn new(content: &str, model: SupportedModel) -> Self {
        let mut prompt = Prompt::new(model);
        prompt.push(content);
        prompt.push("You are an expert in SEO and keyword research. Analyze the article above and provide a list of 6 keywords that capture the main points and supporting evidence.");

        Self { prompt }
    }

    pub fn include_examples(mut self) -> Self {
        self.prompt.push("Examples of good responses are:");
        let selections = select_examples(KEYWORD_EXAMPLES.to_vec(), 3);

        for selection in selections {
            let ex = selection
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            self.prompt.push_example(JSONResponse::from(ex).to_string());
        }

        self
    }
}

impl_prompt_template!(KeywordTemplate);

pub struct GenreTemplate {
    prompt: Prompt,
}
impl GenreTemplate {
    pub fn new(content: &str, model: SupportedModel) -> Self {
        let mut prompt = Prompt::new(model);
        prompt.push(content);
        prompt.push("You are a literary critic. Which genre best applies to the article above?");
        let genre_strings = ArticleGenre::iter()
            .map(|genre| genre.to_string())
            .collect::<Vec<String>>();
        prompt.push(
            format!(
                "Select a genre from the following list: {}",
                genre_strings.join(", ")
            )
            .as_str(),
        );

        Self { prompt }
    }
}

impl_prompt_template!(GenreTemplate);
