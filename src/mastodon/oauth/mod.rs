use serde::{Deserialize, Serialize};

use crate::mastodon;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RegisteredApp {
    pub instance_fqdn: String,
    pub client_id: String,
    pub client_secret: String,
    pub name: String,
    pub redirect_uri: String,
    pub website: Option<String>
}

pub fn register_app(instance_fqdn: &str, app_name: &str) -> Result<RegisteredApp, mastodon::RequestError> {
    let resp: Result<ureq::Response, ureq::Error> = ureq::post(&format!("https://{}/api/v1/apps", instance_fqdn))
        .send_form(
            &[("client_name", app_name),
              ("redirect_uris", "urn:ietf:wg:oauth:2.0:oob"),
              ("scopes", "read write push")]);
    match resp {
        Ok(resp) =>
            resp.into_json().map_err(mastodon::RequestError::JsonError),
        Err(ureq::Error::Status(code, response)) =>
            Err(mastodon::RequestError::HttpError(code,response.status_text().to_string())),
        Err(_) =>
            Err(mastodon::RequestError::HttpError(0, "Transport error".to_string()))
    }
}

pub fn gen_authorize_url(instance_fqdn: &str, app:&RegisteredApp) -> String {
    format!("https://{}/oauth/authorize?response_type=code&client_id={}\
             &redirect_uri=urn:ietf:wg:oauth:2.0:oob\
             &scope=read+write+follow+push", instance_fqdn, app.client_id)
}
