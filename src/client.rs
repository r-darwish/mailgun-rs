use reqwest::Client as ReqwestClient;
use reqwest::header;
use std::borrow::Cow;
use super::domain::Domain;

pub struct Client {
    client: ReqwestClient,
}

impl Client {
    pub fn new<'a, S>(key: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        let mut headers = header::Headers::new();
        headers.set(header::Authorization(header::Basic {
            username: "api".into(),
            password: Some(key.into().to_string()),
        }));

        Client {
            client: ReqwestClient::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    pub fn domain<'a, S>(&'a self, domain: S) -> Domain
    where
        S: Into<Cow<'a, str>>,
    {
        Domain::new(&self.client, domain.into())
    }
}
