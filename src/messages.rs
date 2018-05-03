cpfuse std::borrow::Cow;
use super::domain::Domain;
use reqwest::Result;

#[derive(Serialize)]
struct Message {
    to: String,
    subject: String,
    from: String,
    text: String,
}

pub struct MessageBuilder<'a> {
    to: Option<Cow<'a, str>>,
    from: Option<Cow<'a, str>>,
    subject: Option<Cow<'a, str>>,
    domain: &'a Domain<'a>,
}

impl<'a> MessageBuilder<'a> {
    pub fn new(domain: &'a Domain<'a>) -> Self {
        Self {
            to: None,
            from: None,
            subject: None,
            domain: domain,
        }
    }

    pub fn from<S>(mut self, from: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.from = Some(from.into());
        self
    }

    pub fn to<S>(mut self, to: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.to = Some(to.into());
        self
    }

    pub fn subject<S>(mut self, subject: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.subject = Some(subject.into());
        self
    }

    pub fn send(self) -> Result<()> {
        let message = Message {
            to: self.to.map_or(String::new(), |to| to.into()),
            subject: self.subject.map_or(String::new(), |subject| subject.into()),
            from: self.from.map_or(String::new(), |from| from.into()),
            text: String::from(""),
        };

        let mut response = self.domain.post().form(&message).send()?;
        println!("{} - {}", response.status(), response.text().unwrap());
        Ok(())
    }
}
