use reqwest::{Response, StatusCode};

error_chain!{
    foreign_links {
        Reqwest(::reqwest::Error);
    }

    errors {
        Mailgun(code: StatusCode, text: String) {
            description("Mailgun error")
            display("Mailgun error: {} - '{}'", code, text)
        }
    }
}

pub trait MailgunResult {
    fn result(self) -> Result<Response>;
}

impl MailgunResult for Response {
    fn result(mut self) -> Result<Response> {
        if self.status().is_success() {
            return Ok(self);
        }

        Err(ErrorKind::Mailgun(self.status(), self.text()?).into())
    }
}
