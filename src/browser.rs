use anyhow::{anyhow, Result};
use chromiumoxide::{
    fetcher::BrowserFetcherRevisionInfo, Browser, BrowserConfig, BrowserFetcher,
    BrowserFetcherOptions, Page,
};
use std::path::Path;
use tokio::time::{sleep, Duration};
use tokio_stream::StreamExt;
use tracing::debug;

/// Starts the browser and returns a handle to it.
///
/// # Arguments
///
/// * `browser_path` - The path to the browser executable (will be downloaded if not found).
/// * `user_data_dir` - The path to the user data directory (will be created if not found).
/// * `headless` - Whether to run the browser in headless mode.
///
/// # Errors
///
/// * If the browser executable cannot be found or downloaded.
/// * If the user data directory cannot be created.
/// * If the browser cannot be launched.
/// * If the browser handler cannot be spawned.
pub async fn init(browser_path: &Path, user_data_dir: &Path, headless: bool) -> Result<Browser> {
    std::fs::create_dir_all(browser_path)?;
    std::fs::create_dir_all(user_data_dir)?;

    let browser_info = ensure_browser(browser_path).await?;

    let mut config = BrowserConfig::builder()
        .user_data_dir(user_data_dir)
        .chrome_executable(browser_info.executable_path);

    if headless {
        config = config.with_head();
    }

    let (browser, mut handler) = Browser::launch(config.build().map_err(|e| anyhow!(e))?).await?;

    tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                debug!("Browser handler error: {:?}", h);
                break;
            }
        }
    });

    Ok(browser)
}

async fn ensure_browser(path: &Path) -> Result<BrowserFetcherRevisionInfo> {
    let fetcher = BrowserFetcher::new(BrowserFetcherOptions::builder().with_path(path).build()?);

    Ok(fetcher.fetch().await?)
}

/// Waits for the page to navigate or for 5 seconds to pass.
///
/// # Arguments
///
/// * `page` - The page to wait for.
pub async fn wait_for_page(page: &Page) {
    tokio::select! {
        _ = page.wait_for_navigation() => {},
        _ = sleep(Duration::from_secs(5)) => {},
    }
}
