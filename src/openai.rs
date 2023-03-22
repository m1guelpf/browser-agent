use anyhow::{anyhow, Result};
use async_openai::{
    types::{ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, Role},
    Client,
};
use indoc::formatdoc;
use url::Url;

use crate::Action;

pub struct Conversation {
    goal: String,
    client: Client,
    url: Option<Url>,
    messages: Vec<ChatCompletionRequestMessage>,
}

impl Conversation {
    pub fn new(goal: String) -> Self {
        Self {
            goal,
            url: None,
            client: Client::new(),
            messages: vec![ChatCompletionRequestMessage {
                name: None,
                role: Role::System,
                content: formatdoc!("
                    You are an agent controlling a browser. You are given an objective that you are trying to achieve, the URL of the current website, and a simplified markup description of the page contents, which looks like this:

                    ```
                        <p id=0>text</p>
                        <link id=1 href=\"link url\">text</link>
                        <button id=2>text</button>
                        <input id=3>placeholder</input>
                        <img id=4 alt=\"image description\"/>
                    ```

                    You must respond with ONLY one of the following commands AND NOTHING ELSE:
                        - CLICK X - click on a given element. You can only click on links, buttons, and inputs!
                        - TYPESUBMIT X \"TEXT\" -  type the specified text into the input with id X and press ENTER
                        - END \"TEXT\" - Respond to the user with the specified text once you have completed the objective
                "),
        }]}
    }

    pub async fn request_action(&mut self, url: &str, page_content: &str) -> Result<Action> {
        self.enforce_context_length(url)?;

        self.messages.push(ChatCompletionRequestMessage {
            name: None,
            role: Role::User,
            content: format!(
                "OBJECTIVE: {}\nCURRENT URL: {url}\nPAGE CONTENT:{page_content}",
                self.goal
            ),
        });

        let response = self
            .client
            .chat()
            .create(
                CreateChatCompletionRequestArgs::default()
                    .model("gpt-4")
                    .temperature(0.7f32)
                    .max_tokens(100u16)
                    .messages(self.messages.clone())
                    .build()?,
            )
            .await?;

        let message = &response
            .choices
            .get(0)
            .ok_or_else(|| anyhow!("No choices returned from OpenAI.",))?
            .message;

        self.messages.push(ChatCompletionRequestMessage {
            name: None,
            role: message.role.clone(),
            content: message.content.clone(),
        });

        dbg!(&message.content);

        message.content.clone().try_into()
    }

    fn enforce_context_length(&mut self, url: &str) -> Result<()> {
        let new_url = Url::parse(url)?;

        if self.url.as_ref().map(Url::host) != Some(new_url.host()) {
            self.messages = self.messages.drain(..1).collect();
        }

        self.url = Some(new_url);
        Ok(())
    }
}
