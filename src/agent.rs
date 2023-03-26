use anyhow::{anyhow, bail, Result};

/// Actions that can be taken by the browser.
#[derive(Debug)]
pub enum Action {
    /// Click on an element.
    /// The usize is the id of the element.
    Click(usize),
    /// Respond to the user with the given text.
    /// The String is the text to respond with.
    Answer(String),

    /// Type the given text into the given element and press ENTER.
    /// The usize is the id of the element, and the String is the text to type.
    Type(usize, String),
}

impl TryFrom<String> for Action {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        let mut parts = value.split_whitespace();
        let command = parts.next().ok_or_else(|| anyhow!("No command."))?;

        match command {
            "CLICK" => {
                let id = parts.next().unwrap().parse()?;

                Ok(Self::Click(id))
            }
            "TYPE" => {
                let id = parts.next().unwrap().parse()?;

                let text = parts
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim_matches('"')
                    .to_string();

                Ok(Self::Type(id, text))
            }
            "ANSWER" => {
                let text = parts
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim_matches('"')
                    .to_string();

                Ok(Self::Answer(text))
            }
            _ => bail!("Unknown command, got {command}"),
        }
    }
}
