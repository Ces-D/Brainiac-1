use std::path::PathBuf;

use brainiac::model::SupportedModel;
use clap::{builder::NonEmptyStringValueParser, value_parser, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about="A tool to generate metadata for markdown files", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Generate the metdata for a markdown file")]
    Generate {
        #[arg(short, long, help = "The path to the SOURCE markdown file", required = true, value_parser=value_parser!(PathBuf))]
        source_path: PathBuf,
        #[arg(short, long, help = "The path to the OUTPUT directory", required = true, value_parser=value_parser!(PathBuf))]
        output_dir_path: Option<PathBuf>,
        #[arg(short, long, help = "The name of the model to use")]
        model: Option<SupportedModel>,
        #[arg(short,long, help="The name of the file author", required=true, value_parser=NonEmptyStringValueParser::new())]
        author: String,
    },
}
