use std::borrow::Cow;
use super::domain::Domain;
use super::errors::*;

#[derive(Serialize, Builder)]
pub struct Message<'a> {
    #[builder(setter(into), default)]
    #[serde(skip_serializing_if = "str::is_empty")]
    to: Cow<'a, str>,

    #[builder(setter(into), default)]
    #[serde(skip_serializing_if = "str::is_empty")]
    subject: Cow<'a, str>,

    #[builder(setter(into))]
    from: Cow<'a, str>,

    #[builder(setter(into), default)]
    #[serde(skip_serializing_if = "str::is_empty")]
    text: Cow<'a, str>,

    #[builder(setter(into), default)]
    #[serde(skip_serializing_if = "str::is_empty")]
    html: Cow<'a, str>,

    #[serde(skip)]
    domain: &'a Domain<'a>,
}

impl<'a> Message<'a> {
    pub fn send(self) -> Result<()> {
        self.domain.post().form(&self).send()?.result()?;
        Ok(())
    }
}
