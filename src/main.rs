#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::path::Path;
use anyhow::{anyhow, Result};
use clap::Parser;
use tracing::{debug, info, trace, Level};
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

use browser_agent::{browser, translate, Action, Conversation};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The goal for the agent to achieve
    goal: String,

    /// Whether to show the browser window. Warning: this makes the agent more unreliable.
    #[arg(long)]
    visual: bool,

    /// Set the verbosity level, can be used multiple times
    #[arg(short, action = clap::ArgAction::Count)]
    verbosity: u8,

    /// Whether to include text from the page in the prompt
    #[arg(long)]
    include_page_content: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let args = Cli::parse();

    tracing_subscriber::registry()
        .with(
            EnvFilter::from_default_env().add_directive(
                format!(
                    "browser_agent={}",
                    match args.verbosity {
                        0 => Level::WARN,
                        1 => Level::INFO,
                        2 => Level::DEBUG,
                        3 => Level::TRACE,
                        _ => panic!("Invalid verbosity level."),
                    }
                )
                .parse()?,
            ),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut conversation = Conversation::new(args.goal);
    let mut browser = browser::init(
        Path::new("./browser"),
        Path::new("./user_data"),
        args.visual,
    )
    .await?;

    let page = browser.new_page("https://duckduckgo.com/").await?;

    loop {
        browser::wait_for_page(&page).await;

        let url = page.url().await?.expect("Page should have a URL");
        let elements = page.find_elements("p, button, input, a, img").await?;

        info!("Current URL: {}", url);
        debug!("Found {} elements.", elements.len());

        let page_content = translate(&elements, args.include_page_content).await?;
        let action = conversation.request_action(&url, &page_content).await?;

        match action {
            Action::Click(id) => {
                let element = elements
                    .get(id)
                    .ok_or_else(|| anyhow!("Failed to find element."))?;

                info!(
                    "Clicking on \"{}\".",
                    element
                        .inner_text()
                        .await?
                        .expect("Target should have text.")
                );

                element.click().await?;
            }
            Action::Type(id, text) => {
                let element = elements
                    .get(id)
                    .ok_or_else(|| anyhow!("Failed to find element."))?;

                info!("Typing \"{}\" into input.", text);

                element.type_str(text).await?;
                element.press_key("Enter").await?;
            }
            Action::Answer(text) => {
                println!("{text}");
                break;
            }
        };
    }

    browser.close().await?;
    trace!("Browser closed.");
    Ok(())
}
