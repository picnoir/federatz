use std::io::Error;

pub mod accounts;
pub mod oauth;
pub mod timelines;

#[derive(Debug)]
pub enum RequestError {
    HttpError(u16, String),
    JsonError(Error)
}
