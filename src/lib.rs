use std::borrow::Cow;

pub struct Client<'a> {
    key: Cow<'a, str>,
}

pub struct MailBuilder<'a> {
    client: &'a mut Client<'a>,
    to: Option<Cow<'a, str>>,
    subject: Option<Cow<'a, str>>,
}

impl<'a, 'b> MailBuilder<'a> {
    fn new(client: &'a mut Client<'a>) -> Self {
        Self {
            client: client,
            to: None,
            subject: None,
        }
    }

    pub fn send(mut self) {
    }
}

impl<'a> Client<'a> {
    pub fn new<S>(key: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Client { key: key.into() }
    }

    pub fn mail(&'a mut self) -> MailBuilder<'a> {
        MailBuilder::new(self)
    }
}
