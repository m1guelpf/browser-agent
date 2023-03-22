#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::Path;

use browser_agent::{browser, translate, Action, Conversation};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The goal for the agent to achieve
    goal: String,

    /// Whether to show the browser window
    #[arg(long)]
    inspect: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let mut conversation = Conversation::new(args.goal);
    let mut browser = browser::init(
        Path::new("./browser"),
        Path::new("./user_data"),
        args.inspect,
    )
    .await?;

    let page = browser.new_page("https://duckduckgo.com/").await?;

    loop {
        let url = page.url().await?.expect("Page should have a URL");
        let elements = page
            .wait_for_navigation()
            .await?
            .find_elements("p, button, input, a, img")
            .await?;

        let page_content = translate(&elements).await?;
        let action = conversation.request_action(&url, &page_content).await?;

        match action {
            Action::Click(id) => {
                let element = elements
                    .get(id)
                    .ok_or_else(|| anyhow!("Failed to find element."))?;

                element.click().await?;
            }
            Action::TypeSubmit(id, text) => {
                let element = elements
                    .get(id)
                    .ok_or_else(|| anyhow!("Failed to find element."))?;

                element.type_str(text).await?;
                element.press_key("Enter").await?;
            }
            Action::End(text) => {
                println!("{text}");
                break;
            }
        };
    }

    browser.close().await?;
    Ok(())
}
