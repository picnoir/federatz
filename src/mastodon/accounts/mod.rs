use serde::{Deserialize, Serialize};

use crate::mastodon;


#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UserAccount {
    pub id: String,
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UserAccountWithToken {
    pub user_account: UserAccount,
    pub token: mastodon::oauth::AuthToken
}

pub fn verify_credentials(instance_fqdn: &str, token: mastodon::oauth::AuthToken) -> Result<UserAccountWithToken, mastodon::RequestError> {
    let resp: Result<ureq::Response, ureq::Error> = ureq::get(&format!("https://{}/api/v1/accounts/verify_credentials", instance_fqdn))
        .set("Authorization", &format!("{} {}", &token.token_type, &token.access_token))
        .call();
    match resp {
        Ok(resp) => {
            let user_account: UserAccount = resp.into_json().map_err(mastodon::RequestError::JsonError)?;
            Ok(UserAccountWithToken {
                token: token,
                user_account
            })
        }
        Err(ureq::Error::Status(code, response)) =>
            Err(mastodon::RequestError::HttpError(code, response.status_text().to_string())),
        Err(_) =>
            Err(mastodon::RequestError::HttpError(0, "Transport error".to_string())),
    }

}

pub fn deserialize_user_account_str(serialized_user_account_without_token: &str, token: mastodon::oauth::AuthToken) -> Result<UserAccountWithToken, serde_json::Error> {
    let user_account: UserAccount = serde_json::from_str(serialized_user_account_without_token)?;
    Ok(UserAccountWithToken {
        user_account,
        token
    })
}

#[cfg(test)]
mod test {
    use crate::mastodon::accounts::{UserAccount, verify_credentials};

    static PLEROMA_JSON_FIXTURE: &'static str = r#"
{
    "acct": "Ninjatrappeur",
    "avatar": "https://social.alternativebit.fr/media/3f9ad2d6f473c954506f404de5a3636905035f32ec9f1f07deca0a72e88ac3e8.blob",
    "avatar_static": "https://social.alternativebit.fr/media/3f9ad2d6f473c954506f404de5a3636905035f32ec9f1f07deca0a72e88ac3e8.blob",
    "bot": false,
    "created_at": "2018-05-26T13:07:04.000Z",
    "display_name": "⠴Ninjatrappeur⠦",
    "emojis": [],
    "fields": [],
    "follow_requests_count": 0,
    "followers_count": 615,
    "following_count": 191,
    "fqn": "Ninjatrappeur@social.alternativebit.fr",
    "header": "https://social.alternativebit.fr/media/b5f2cf7e-7892-4692-b0c8-157818ad1b87/57CF8FB0FE2143FF1267A90BD3217D8D433FA2381C56E36264352A3EA0F17838.png",
    "header_static": "https://social.alternativebit.fr/media/b5f2cf7e-7892-4692-b0c8-157818ad1b87/57CF8FB0FE2143FF1267A90BD3217D8D433FA2381C56E36264352A3EA0F17838.png",
    "id": "1",
    "locked": false,
    "note": "Rural Hacker<br/><br/>Free Software/NixOS/Guix/Haskell/Rust/C | Cycling/Bird Spotting/Music/Trains<br/><br/>E12D 4FFE F5C4 AD27 1345 0A8F 6C7E 9E38 B61E 7F42<br/><br/><a href=\"https://alternativebit.fr\">https://alternativebit.fr</a><br/><br/>Boost != Endorsement",
    "pleroma": {
        "accepts_chat_messages": true,
        "allow_following_move": true,
        "also_known_as": [],
        "ap_id": "https://social.alternativebit.fr/users/Ninjatrappeur",
        "background_image": "https://social.alternativebit.fr/media/4976ce4e-b9e3-442a-b196-a267584d93fe/682EA0F6C436DAC1112BDA1933F74170C201BD24670B5588F03634F06B1BE9F6.png",
        "chat_token": "SFMyNTY.g2gDbQAAAAExbgYAtyQYHYIBYgABUYA.-wkIaeh9S7IO9yj7E0ymwMQ1pL4Wovciz6FYX0xcjkA",
        "deactivated": false,
        "email": "felix@alternativebit.fr",
        "favicon": null,
        "hide_favorites": true,
        "hide_followers": false,
        "hide_followers_count": false,
        "hide_follows": false,
        "hide_follows_count": false,
        "is_admin": true,
        "is_confirmed": true,
        "is_moderator": true,
        "notification_settings": {
            "block_from_strangers": false,
            "hide_notification_contents": false
        },
        "relationship": {},
        "settings_store": {},
        "skip_thread_containment": false,
        "tags": [],
        "unread_conversation_count": 67,
        "unread_notifications_count": 0
    },
    "source": {
        "fields": [],
        "note": "Rural Hacker\n\nFree Software/NixOS/Guix/Haskell/Rust/C | Cycling/Bird Spotting/Music/Trains\n\nE12D 4FFE F5C4 AD27 1345 0A8F 6C7E 9E38 B61E 7F42\n\nhttps://alternativebit.fr\n\nBoost != Endorsement",
        "pleroma": {
            "actor_type": "Person",
            "discoverable": false,
            "no_rich_text": false,
            "show_role": true
        },
        "privacy": "public",
        "sensitive": false
    },
    "statuses_count": 1297,
    "url": "https://social.alternativebit.fr/users/Ninjatrappeur",
    "username": "Ninjatrappeur"
}"#;


    fn expected_account () -> UserAccount {
        UserAccount {
            id: "1".to_string(),
            username: "Ninjatrappeur".to_string(),
            display_name: "⠴Ninjatrappeur⠦".to_string(),
            locked: false,
            bot: false,
            created_at: "2018-05-26T13:07:04.000Z".to_string(),
            note: "Rural Hacker<br/><br/>Free Software/NixOS/Guix/Haskell/Rust/C | Cycling/Bird Spotting/Music/Trains<br/><br/>E12D 4FFE F5C4 AD27 1345 0A8F 6C7E 9E38 B61E 7F42<br/><br/><a href=\"https://alternativebit.fr\">https://alternativebit.fr</a><br/><br/>Boost != Endorsement".to_string(),
            url: "https://social.alternativebit.fr/users/Ninjatrappeur".to_string(),
            avatar: "https://social.alternativebit.fr/media/3f9ad2d6f473c954506f404de5a3636905035f32ec9f1f07deca0a72e88ac3e8.blob".to_string(),
            avatar_static: "https://social.alternativebit.fr/media/3f9ad2d6f473c954506f404de5a3636905035f32ec9f1f07deca0a72e88ac3e8.blob".to_string(),
            header: "https://social.alternativebit.fr/media/b5f2cf7e-7892-4692-b0c8-157818ad1b87/57CF8FB0FE2143FF1267A90BD3217D8D433FA2381C56E36264352A3EA0F17838.png".to_string(),
            header_static: "https://social.alternativebit.fr/media/b5f2cf7e-7892-4692-b0c8-157818ad1b87/57CF8FB0FE2143FF1267A90BD3217D8D433FA2381C56E36264352A3EA0F17838.png".to_string(),
            followers_count: 615,
            following_count: 191,
            statuses_count: 1297,
        }
    }

    #[test]
    fn test_parse_pleroma_user () {
        let got: UserAccount = serde_json::from_str(&PLEROMA_JSON_FIXTURE).unwrap();
        assert_eq!(got, expected_account());
    }
}
