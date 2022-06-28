use std::io::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisteredApp {
    client_id: String,
    client_secret: String,
    id: String,
    name: String,
    redirect_uri: String,
    website: Option<String>
}

#[derive(Debug)]
pub enum RequestError {
    HttpError(u16, String),
    JsonError(Error)

}

pub fn register_app(instance_fqdn: &str, app_name: &str) -> Result<RegisteredApp, RequestError> {
    let resp: Result<ureq::Response, ureq::Error> = ureq::post(&format!("https://{}/api/v1/apps", instance_fqdn))
        .send_form(
            &[("client_name", app_name),
              ("redirect_uris", "urn:ietf:wg:oauth:2.0:oob"),
              ("scopes", "read write push")]);
    let io_error_to_request_error = |err| RequestError::JsonError(err);
    match resp {
        Ok(resp) => resp.into_json().map_err(io_error_to_request_error),
        Err(ureq::Error::Status(code, response)) => Err(RequestError::HttpError(code,response.status_text().to_string())),
        Err(_) => Err(RequestError::HttpError(0, "Transport error".to_string()))
    }
}
