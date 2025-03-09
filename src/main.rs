//! wat is a command line tool that uses llms to get information about a file on disk.
use std::env;

use clap::builder::ArgPredicate;
use clap::Parser;
use eyre::Result;

mod text_gen;

const DEFAULT_BASE_URL: &str = "https://api.anthropic.com/v1/chat/completions";
const DEFAULT_MODEL: &str = "claude-3-5-sonnet-20241022";
const DEFAULT_PROMPT: &str = "You are a helpful assistant that uses information such as the path to a file to provide information on the file such as what programs its used with.";

#[derive(Debug, clap::Parser)]
struct Args {
    #[clap(default_value_if("show_config_path", ArgPredicate::IsPresent, ""))]
    file_path: String,
    #[clap(short, long)]
    model: Option<String>,
    #[clap(long, exclusive = true)]
    show_config_path: bool,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Default)]
struct Config {
    #[serde(default)]
    base_url: Option<String>,
    #[serde(default)]
    model: Option<String>,
    #[serde(default)]
    api_key: Option<String>,
    #[serde(default)]
    prompt: Option<String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.show_config_path {
        let config_path = confy::get_configuration_file_path("wtfile", Some("config"))?;
        println!("{}", config_path.display());
        return Ok(());
    }

    let config: Config = confy::load("wat", None)?;

    let base_url = config.base_url.as_deref().unwrap_or(DEFAULT_BASE_URL);
    let api_key = env::var("OPENAI_API_KEY").ok().or(config.api_key).unwrap();
    let model = args
        .model
        .as_deref()
        .unwrap_or(config.model.as_deref().unwrap_or(DEFAULT_MODEL));
    let prompt = config.prompt.as_deref().unwrap_or(DEFAULT_PROMPT);

    let response = text_gen::TextGenClient::new(base_url, &api_key)
        .chat_completions()
        .model(model)
        .messages(&[
            text_gen::Message::system(prompt),
            text_gen::Message::user(&args.file_path),
        ])
        .send()
        .await?;

    println!("{}", response.choices[0].message.content);
    Ok(())
}
