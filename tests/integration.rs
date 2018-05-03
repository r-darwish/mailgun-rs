extern crate mailgun;

use mailgun::Client;
use std::env;

#[test]
fn send_message() {
    let client = Client::new(env::var("MAILGUN_API_KEY").unwrap());
    let domain = client.domain(env::var("MAILGUN_DOMAIN").unwrap());
    domain
        .message()
        .from("Someone <someone@somedomain.com>")
        .to(env::var("MAILGUN_TO").unwrap())
        .subject("Testing Message")
        .text("Hi")
        .build()
        .unwrap()
        .send()
        .unwrap();
}
