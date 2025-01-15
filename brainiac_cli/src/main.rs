use brainiac::{append_metadata, create_output_file_name, BrainiacAppend};
use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use std::{error::Error, io::Write};

mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Cli::parse();

    match app.command {
        Commands::Generate {
            source_path,
            output_dir_path,
            model,
            author,
        } => {
            let params = BrainiacAppend {
                source_path,
                output_dir_path,
                model,
                author,
            };
            match append_metadata(params).await {
                Ok(metadata) => {
                    let stdout = std::io::stdout(); // get the global stdout entity
                    let mut handle = stdout.lock(); // acquire a lock on it
                    writeln!(handle, "{}", "Successfully appeneded metadata".green())?;
                    writeln!(handle, "{:<10}{}", "Title", metadata.title)?;
                    writeln!(
                        handle,
                        "{:<10}{}",
                        "File",
                        create_output_file_name(&metadata.slug).to_string_lossy()
                    )?;
                    Ok(())
                }
                Err(_) => todo!(),
            }
        }
    }
}
