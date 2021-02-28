use crate::Output;
use log::{debug, error};
use reqwest::blocking::Client;
use serde::Serialize;

pub struct Slack {
    hook: String,
    client: Client,
}
#[derive(Debug, Serialize)]
pub struct Message {
    text: Option<String>,
}

impl Message {
    pub fn new() -> Self {
        Self { text: None }
    }

    pub fn text(&mut self, text: &str) -> &mut Message {
        self.text = Some(text.to_owned());
        self
    }
}

impl Output for Slack {
    fn execute(&self, message: String) {
        // TODO: fix error handling, currently fire and forget
        let result = self
            .client
            .post(&self.hook)
            .json(Message::new().text(message.as_ref()))
            .send();

        match result {
            Ok(_) => {
                debug!("successfully sent message to slack");
            }
            Err(e) => {
                error!("{:#?}", e);
            }
        }
    }
}

impl Slack {
    pub fn new(hook: String) -> Slack {
        Slack {
            hook,
            client: Client::new(),
        }
    }
}
