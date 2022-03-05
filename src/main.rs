use clap::{Parser, Subcommand};
use config::Config;
use dialoguer::Input;
use std::error::Error;

mod api;
mod config;
mod rofi;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Add a new todo")]
    Add,
    #[clap(about = "Configure the notion variables")]
    Configure,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {} => {
            let config = Config::try_read()?.expect("you have not configured the credentials yet");
            if let Some(title) = rofi::get_title().await?.filter(|title| !title.is_empty()) {
                api::create_todo(&config, &title).await?;
            }
        }
        Commands::Configure => {
            let secret: String = Input::new().with_prompt("Secret").interact_text()?;
            let database_id: String = Input::new().with_prompt("Database ID").interact_text()?;
            let config = Config::new(secret, database_id);
            config.write()?;
        }
    }

    Ok(())
}
