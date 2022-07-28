use serde::{Deserialize, Serialize};

use crate::mastodon;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UserAccount {
    pub id: String,
    pub token: String,
    pub username: String,
    pub display_name: String,
    pub locked: bool,
    pub bot: bool,
    pub created_at: String,
    pub note: String,
    pub url: String,
    pub avatar: String,
    pub avatar_static: String,
    pub header: String,
    pub header_static: String,
    pub followers_count: u32,
    pub following_count: u32,
    pub statuses_count: u32,
}

pub fn verify_credentials(instance_fqdn: &str, authorization_token: &str) -> Result<UserAccount, mastodon::RequestError> {
    let resp: Result<ureq::Response, ureq::Error> = ureq::get(&format!("https://{}/api/v1/accounts/verify_credentials", instance_fqdn))
        .set("Authorization", authorization_token)
        .call();
    match resp {
        Ok(resp) =>
            resp.into_json().map_err(mastodon::RequestError::JsonError),
        Err(ureq::Error::Status(code, response)) =>
            Err(mastodon::RequestError::HttpError(code, response.status_text().to_string())),
        Err(_) =>
            Err(mastodon::RequestError::HttpError(0, "Transport error".to_string())),
    }

}
