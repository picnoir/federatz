use serde::{Deserialize, Serialize};

use crate::mastodon;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RegisteredApp {
    pub client_id: String,
    pub client_secret: String,
    pub name: String,
    pub redirect_uri: String,
    pub website: Option<String>
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RegisteredAppWithInstanceFqdn {
    pub registered_app: RegisteredApp,
    pub instance_fqdn: String
}

pub fn register_app(instance_fqdn: &str, app_name: &str) -> Result<RegisteredAppWithInstanceFqdn, mastodon::RequestError> {
    let resp: Result<ureq::Response, ureq::Error> = ureq::post(&format!("https://{}/api/v1/apps", instance_fqdn))
        .send_form(
            &[("client_name", app_name),
              ("redirect_uris", "urn:ietf:wg:oauth:2.0:oob"),
              ("scopes", "read write push follow admin")]);
    match resp {
        Ok(resp) => {
            let registered_app: RegisteredApp = resp.into_json().map_err(mastodon::RequestError::JsonError)?;
            Ok(RegisteredAppWithInstanceFqdn {
                instance_fqdn: instance_fqdn.to_string(),
                registered_app
            })
        }
        Err(ureq::Error::Status(code, response)) =>
            Err(mastodon::RequestError::HttpError(code,response.status_text().to_string())),
        Err(_) =>
            Err(mastodon::RequestError::HttpError(0, "Transport error".to_string()))
    }
}

pub fn gen_authorize_url(app: &RegisteredAppWithInstanceFqdn) -> String {
    format!("https://{}/oauth/authorize?response_type=code&client_id={}\
             &redirect_uri=urn:ietf:wg:oauth:2.0:oob\
             &scope=read+write+follow+admin+push", app.instance_fqdn, app.registered_app.client_id)
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AuthToken {
    pub access_token: String,
    pub created_at: u64,
    pub expires_in: u64,
    pub id: u64,
    pub me: String,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String
}

pub fn generate_token(app: &RegisteredAppWithInstanceFqdn, auth_code: &str) -> Result<AuthToken, mastodon::RequestError> {
    let resp: Result<ureq::Response, ureq::Error> = ureq::post(&format!("https://{}/oauth/token", app.instance_fqdn))
        .send_form(
            &[("grant_type", "authorization_code"),
              ("client_id", &app.registered_app.client_id),
              ("client_secret", &app.registered_app.client_secret),
              ("redirect_url", "urn:ietf:wg:oauth:2.0:oob"),
              ("scope", "read write follow push"),
              ("code", auth_code)
            ]);
    match resp {
        Ok(resp) =>
            resp.into_json().map_err(mastodon::RequestError::JsonError),
        Err(ureq::Error::Status(code, response)) =>
            Err(mastodon::RequestError::HttpError(code, response.status_text().to_string())),
        Err(_) =>
            Err(mastodon::RequestError::HttpError(0, "Transport error".to_string()))
    }
}

#[cfg(test)]
mod test {
    use crate::mastodon::oauth::{RegisteredApp, AuthToken, register_app, generate_token};

    static pleroma_register_app_json_fixture: &'static str = r#"
{
    "client_id": "yUy22iiE1Ql1RYw3GFSX-pCyp4zn4pt3f5KVO4l8O3c",
    "client_secret": "NT_ypYpC25xhPKQYIzgBzoEpqRRBTwQHUyYYzJbz2BE",
    "id": "59545",
    "name": "test-app",
    "redirect_uri": "urn:ietf:wg:oauth:2.0:oob",
    "website": null
}"#;

    static pleroma_generate_token_json_fixture: &'static str = r#"
{
    "access_token": "8YA7xlkn9auNTeCZl9Z7lvfnDOEZu4VcatT1p1plqms",
    "created_at": 1656412658,
    "expires_in": 3153600000,
    "id": 73525,
    "me": "https://social.alternativebit.fr/users/Ninjatrappeur",
    "refresh_token": "q_flooiKgptxoirXjF_yAdCGhVTXjHpGKqhb7hs6mfs",
    "scope": "read write push",
    "token_type": "Bearer"
}"#;

    fn expected_app () -> RegisteredApp {
        RegisteredApp {
            client_id: "yUy22iiE1Ql1RYw3GFSX-pCyp4zn4pt3f5KVO4l8O3c".to_string(),
            client_secret: "NT_ypYpC25xhPKQYIzgBzoEpqRRBTwQHUyYYzJbz2BE".to_string(),
            name: "test-app".to_string(),
            redirect_uri: "urn:ietf:wg:oauth:2.0:oob".to_string(),
            website: None
        }
    }

    fn expected_token () -> AuthToken {
        AuthToken {
            access_token: "8YA7xlkn9auNTeCZl9Z7lvfnDOEZu4VcatT1p1plqms".to_string(),
            created_at: 1656412658,
            expires_in: 3153600000,
            id: 73525,
            me: "https://social.alternativebit.fr/users/Ninjatrappeur".to_string(),
            refresh_token: "q_flooiKgptxoirXjF_yAdCGhVTXjHpGKqhb7hs6mfs".to_string(),
            scope: "read write push".to_string(),
            token_type: "Bearer".to_string()
        }
    }

    #[test]
    fn test_parse_pleroma_app () {
        let got: RegisteredApp = serde_json::from_str(&pleroma_register_app_json_fixture).unwrap();
        assert_eq!(got, expected_app());
    }

    #[test]
    fn test_parse_pleroma_token () {
        let got: AuthToken = serde_json::from_str(&pleroma_generate_token_json_fixture).unwrap();
        assert_eq!(got, expected_token());
    }

}
