#[macro_use]
extern crate derive_builder;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

mod client;
mod messages;
mod domain;

pub use client::Client;
