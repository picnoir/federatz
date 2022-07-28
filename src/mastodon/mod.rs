use std::io::Error;

pub mod accounts;
pub mod oauth;

#[derive(Debug)]
pub enum RequestError {
    HttpError(u16, String),
    JsonError(Error)
}
