extern crate mailgun;

use mailgun::Client;

#[test]
fn it_adds_two() {
    let mut client = Client::new("asd");
    let mail = client.mail().send();
}
