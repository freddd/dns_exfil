use crate::Output;
use log::{debug, error};
use reqwest::blocking::Client;
use serde::Serialize;

pub struct Discord {
    hook: String,
    client: Client,
}

#[derive(Debug, Serialize)]
pub struct Message {
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
}

impl Message {
    pub fn new() -> Self {
        Self {
            content: None,
            username: None,
        }
    }

    pub fn content(&mut self, content: &str) -> &mut Message {
        self.content = Some(content.to_owned());
        self
    }

    pub fn username(&mut self, name: &str) -> &mut Message {
        self.username = Some(name.to_owned());
        self
    }
}

impl Output for Discord {
    fn execute(&self, message: String) {
        // TODO: fix error handling, currently fire and forget
        let result = self
            .client
            .post(&self.hook)
            .json(
                Message::new()
                    .username("dns_exfil")
                    .content(message.as_ref()),
            )
            .send();

        match result {
            Ok(_) => {
                debug!("successfully sent message to discord");
            }
            Err(e) => {
                error!("{:#?}", e);
            }
        }
    }
}

impl Discord {
    pub fn new(hook: String) -> Discord {
        Discord {
            hook,
            client: Client::new(),
        }
    }
}
