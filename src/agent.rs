use anyhow::{anyhow, bail, Result};

#[derive(Debug)]
pub enum Action {
    Click(usize),
    Answer(String),
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
            _ => bail!("Unknown command."),
        }
    }
}
