use anyhow::{anyhow, Context, Result};
use chromiumoxide::Element;

/// Translates the given elements into a format GPT-4 can understand.
///
/// # Arguments
///
/// * `elements` - The elements to translate.
/// * `should_include_p` - Whether to include paragraphs in the translation.
///
/// # Errors
///
/// * If the elements cannot be translated.
pub async fn translate(elements: &[Element], should_include_p: bool) -> Result<String> {
    let mut summary = Vec::new();

    for (i, element) in elements.iter().enumerate() {
        let inner_text = element.inner_text().await?;

        match element
            .property("tagName")
            .await
            .context("Failed to get tag name")?
            .ok_or_else(|| anyhow!("Failed to get tag name"))?
            .as_str()
            .context("Failed to get tag name")?
        {
            "BUTTON" => {
                let Some(inner_text) = inner_text else {
                    continue
                };

                summary.push(format!("<button id={i}>{inner_text}</button>"));
            }
            "P" => {
                if !should_include_p {
                    continue;
                }

                let Some(inner_text) = inner_text else {
                    continue
                };

                summary.push(format!("<p id={i}>{inner_text}</p>"));
            }
            "IMG" => {
                let Some(alt_text) = element.attribute("alt").await? else {
                    continue
                };

                summary.push(format!("<img id={i} alt=\"{alt_text}\"/>"));
            }
            "A" => {
                // Do not include links that have headings inside of them. Many websites use this
                // so that you can link directly to a page section
                if element.find_elements("h1,h2,h3,h4,h5,h6").await?.len() > 0 {
                    continue;
                }

                let Some(inner_text) = inner_text else {
                    continue
                };

                let Some(href) = element.attribute("href").await? else {
                    continue
                };

                summary.push(format!("<link id={i} href={href}>{inner_text}<link>",));
            }
            "INPUT" => {
                let Some(placeholder) = element.attribute("placeholder").await? else {
                    continue
                };

                summary.push(format!("<input id={i}>{placeholder}</input>"));
            }
            _ => {}
        }
    }

    Ok(summary.join("\n"))
}
