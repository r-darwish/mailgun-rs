use reqwest::{Client, RequestBuilder};
use std::borrow::Cow;
use super::messages::MessageBuilder;

pub struct Domain<'a> {
    client: &'a Client,
    base_url: String,
}

impl<'a> Domain<'a> {
    pub(crate) fn new(client: &'a Client, domain: Cow<'a, str>) -> Self {
        Domain {
            client: client,
            base_url: format!("https://api.mailgun.net/v3/{}/messages", domain),
        }
    }

    pub fn message<'b: 'a>(&'b self) -> MessageBuilder<'a> {
        let mut builder = MessageBuilder::default();
        builder.domain(&self);
        builder
    }

    pub(crate) fn post(&self) -> RequestBuilder {
        self.client.post(&self.base_url)
    }
}
