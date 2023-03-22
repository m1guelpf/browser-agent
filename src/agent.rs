use anyhow::{anyhow, bail, Result};

#[derive(Debug)]
pub enum Action {
    Click(usize),
    TypeSubmit(usize, String),
    End(String),
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
            "TYPESUBMIT" => {
                let id = parts.next().unwrap().parse()?;

                let text = parts
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim_matches('"')
                    .to_string();

                Ok(Self::TypeSubmit(id, text))
            }
            "END" => {
                let text = parts
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim_matches('"')
                    .to_string();

                Ok(Self::End(text))
            }
            _ => bail!("Unknown command."),
        }
    }
}
