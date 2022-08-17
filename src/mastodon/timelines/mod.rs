use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct Timeline {
    statuses: Vec<Status>
}

#[derive(PartialEq, Debug, Deserialize, Clone)]
pub struct Status {
    account: Account,
    bookmarked: bool,
    content: String,
    created_at: String,
    favourited: bool,
    favourites_count: u64,
    id: String,
    in_reply_to_account_id: Option<String>,
    in_reply_to_id: Option<String>,
    media_attachments: Vec<MediaAttachment>,
    mentions: Vec<Mention>,
    muted: bool,
    pinned: bool,
    reblog: Option<Box<Status>>,
    reblogged: bool,
    reblogs_count: u64,
    replies_count: u64,
    sensitive: bool,
    spoiler_text: String,
    uri: String,
    url: String,
    visibility: String
}

#[derive(PartialEq, Debug, Deserialize, Clone)]
pub struct MediaAttachment {
    blurhash: String,
    description: String,
    id: String,
    preview_url: String,
    remote_url: String,
    text_url: String,
    r#type: String,
    url: String
}

#[derive(PartialEq, Debug, Deserialize, Clone)]
pub struct Mention {
    acct: String,
    id: String,
    url: String,
    username: String
}

#[derive(PartialEq, Debug, Deserialize, Clone)]
pub struct Account {
    acct: String,
    avatar: String,
    avatar_static: String,
    bot: bool,
    created_at: String,
    display_name: String,
    fields: Vec<AccountField>,
    followers_count: u64,
    following_count: u64,
    fqn: String,
    id: String,
    locked: bool,
    note: String,
    statuses_count: u64,
    url: String,
    username: String
}

#[derive(PartialEq, Debug, Deserialize, Clone)]
pub struct AccountField {
    name: String,
    value: String
}

#[cfg(test)]
mod test {
    use crate::mastodon::timelines::*;
    static PLEROMA_JSON_PUBLIC_TIMELINE_FIXTURE: &'static str = r#"
[
    {
        "account": {
            "acct": "JordiGH@mathstodon.xyz",
            "avatar": "https://mathstodon.xyz/system/accounts/avatars/000/013/716/original/c5627ee76d38f8cd.png",
            "avatar_static": "https://mathstodon.xyz/system/accounts/avatars/000/013/716/original/c5627ee76d38f8cd.png",
            "bot": false,
            "created_at": "2018-05-28T00:34:59.000Z",
            "display_name": "JordiGH",
            "emojis": [],
            "fields": [
                {
                    "name": "Pronouns",
                    "value": "any"
                },
                {
                    "name": "Blog",
                    "value": "<a href=\"http://jordi.inversethought.com/about/\">http://jordi.inversethought.com/about/</a>"
                },
                {
                    "name": "GNU/Linux version",
                    "value": "Debian GNU/Linux"
                }
            ],
            "followers_count": 463,
            "following_count": 489,
            "fqn": "JordiGH@mathstodon.xyz",
            "header": "https://social.alternativebit.fr/images/banner.png",
            "header_static": "https://social.alternativebit.fr/images/banner.png",
            "id": "823",
            "locked": true,
            "note": "<p>Coder. Mathematician. Hacker-errant.</p><p>You can also email me at jordigh@octave.org</p><p>This year I&#39;m writing a book on mathematics on Twitch, <a href=\"https://twitch.tv/jordigh\"><span>https://</span><span>twitch.tv/jordigh</span><span></span></a></p><p>Please say something to me before following me! I&#39;ll deny your follow request if we haven&#39;t exchanged any words, and I&#39;ll probably approve it if you at least say hi.</p>",
            "pleroma": {
                "accepts_chat_messages": null,
                "also_known_as": [],
                "ap_id": "https://mathstodon.xyz/users/JordiGH",
                "background_image": null,
                "deactivated": false,
                "favicon": null,
                "hide_favorites": true,
                "hide_followers": false,
                "hide_followers_count": false,
                "hide_follows": false,
                "hide_follows_count": false,
                "is_admin": false,
                "is_confirmed": true,
                "is_moderator": false,
                "relationship": {},
                "skip_thread_containment": false,
                "tags": []
            },
            "source": {
                "fields": [],
                "note": "",
                "pleroma": {
                    "actor_type": "Person",
                    "discoverable": true
                },
                "sensitive": false
            },
            "statuses_count": 8260,
            "url": "https://mathstodon.xyz/@JordiGH",
            "username": "JordiGH"
        },
        "application": null,
        "bookmarked": false,
        "content": "<p>FOOD.<br/>So yesterday, I embarked on an 8-ish hour adventure of making <span class=\"h-card\"><a href=\"https://mathstodon.xyz/@JordiGH\" class=\"u-url mention\">@<span>JordiGH</span></a></span> ’s birthday cake. I used some Japanese methods to make the cake itself, which ended up with an angelfood cake-like texture, except with a very precise amount of egg whipping. I made strawberry jam filling from scratch and maybe should put a little more pectin in it, because it was a bit wet in the end. And buttercream. It was fun and I think it turned out to taste ok, although not very pretty. Next time.</p>",
        "created_at": "2022-08-02T16:52:14.000Z",
        "emojis": [],
        "favourited": false,
        "favourites_count": 0,
        "id": "AM7iejUVqeZiIAYReK",
        "in_reply_to_account_id": null,
        "in_reply_to_id": null,
        "language": null,
        "media_attachments": [
            {
                "blurhash": "UNKmzb_00p9I}+Di9$jF%y%LM*sotixZRPjb",
                "description": "Jordi blowing out his braille candles.",
                "id": "630770650",
                "pleroma": {
                    "mime_type": "image/jpeg"
                },
                "preview_url": "https://content.queer.party/media/media_attachments/files/108/754/216/746/179/434/original/3d697c3985b71698.jpg",
                "remote_url": "https://content.queer.party/media/media_attachments/files/108/754/216/746/179/434/original/3d697c3985b71698.jpg",
                "text_url": "https://content.queer.party/media/media_attachments/files/108/754/216/746/179/434/original/3d697c3985b71698.jpg",
                "type": "image",
                "url": "https://content.queer.party/media/media_attachments/files/108/754/216/746/179/434/original/3d697c3985b71698.jpg"
            }
        ],
        "mentions": [
            {
                "acct": "Not_mermista@queer.party",
                "id": "AGBLDkyQLoL35qrqb2",
                "url": "https://queer.party/@Not_mermista",
                "username": "Not_mermista"
            },
            {
                "acct": "Not_mermista@queer.party",
                "id": "AGBLDkyQLoL35qrqb2",
                "url": "https://queer.party/@Not_mermista",
                "username": "Not_mermista"
            }
        ],
        "muted": false,
        "pinned": false,
        "pleroma": {
            "local": false,
            "pinned_at": null
        },
        "reblog": {
            "account": {
                "acct": "Not_mermista@queer.party",
                "avatar": "https://content.queer.party/media/accounts/avatars/107/747/955/493/272/175/original/4b23c2d85193e1cb.jpeg",
                "avatar_static": "https://content.queer.party/media/accounts/avatars/107/747/955/493/272/175/original/4b23c2d85193e1cb.jpeg",
                "bot": false,
                "created_at": "2022-02-05T23:36:00.000Z",
                "display_name": "Not Mermista",
                "emojis": [],
                "fields": [
                    {
                        "name": "Brain",
                        "value": "Full of bees"
                    }
                ],
                "followers_count": 0,
                "following_count": 0,
                "fqn": "Not_mermista@queer.party",
                "header": "https://social.alternativebit.fr/images/banner.png",
                "header_static": "https://social.alternativebit.fr/images/banner.png",
                "id": "AGBLDkyQLoL35qrqb2",
                "locked": false,
                "note": "<p>Nervously joining Mastodon to post cat pictures. ^_^</p>",
                "pleroma": {
                    "accepts_chat_messages": null,
                    "also_known_as": [],
                    "ap_id": "https://queer.party/users/Not_mermista",
                    "background_image": null,
                    "deactivated": false,
                    "favicon": null,
                    "hide_favorites": true,
                    "hide_followers": false,
                    "hide_followers_count": false,
                    "hide_follows": false,
                    "hide_follows_count": false,
                    "is_admin": false,
                    "is_confirmed": true,
                    "is_moderator": false,
                    "relationship": {},
                    "skip_thread_containment": false,
                    "tags": []
                },
                "source": {
                    "fields": [],
                    "note": "",
                    "pleroma": {
                        "actor_type": "Person",
                        "discoverable": false
                    },
                    "sensitive": false
                },
                "statuses_count": 110,
                "url": "https://queer.party/@Not_mermista",
                "username": "Not_mermista"
            },
            "application": null,
            "bookmarked": false,
            "card": null,
            "content": "<p>FOOD.<br/>So yesterday, I embarked on an 8-ish hour adventure of making <span class=\"h-card\"><a href=\"https://mathstodon.xyz/@JordiGH\" class=\"u-url mention\">@<span>JordiGH</span></a></span> ’s birthday cake. I used some Japanese methods to make the cake itself, which ended up with an angelfood cake-like texture, except with a very precise amount of egg whipping. I made strawberry jam filling from scratch and maybe should put a little more pectin in it, because it was a bit wet in the end. And buttercream. It was fun and I think it turned out to taste ok, although not very pretty. Next time.</p>",
            "created_at": "2022-08-02T16:29:18.000Z",
            "emojis": [],
            "favourited": false,
            "favourites_count": 0,
            "id": "AM7iej8XAMzhC0GtOK",
            "in_reply_to_account_id": null,
            "in_reply_to_id": null,
            "language": null,
            "media_attachments": [
                {
                    "blurhash": "UJHxZ$%#OE$|}r4TM_yCOts,ivIUY7%gS4D*",
                    "description": "Inside crumb of the cake.",
                    "id": "-339402590",
                    "pleroma": {
                        "mime_type": "image/jpeg"
                    },
                    "preview_url": "https://content.queer.party/media/media_attachments/files/108/754/216/976/293/842/original/c097701db9d26c4f.jpg",
                    "remote_url": "https://content.queer.party/media/media_attachments/files/108/754/216/976/293/842/original/c097701db9d26c4f.jpg",
                    "text_url": "https://content.queer.party/media/media_attachments/files/108/754/216/976/293/842/original/c097701db9d26c4f.jpg",
                    "type": "image",
                    "url": "https://content.queer.party/media/media_attachments/files/108/754/216/976/293/842/original/c097701db9d26c4f.jpg"
                }
            ],
            "mentions": [
                {
                    "acct": "JordiGH@mathstodon.xyz",
                    "id": "823",
                    "url": "https://mathstodon.xyz/@JordiGH",
                    "username": "JordiGH"
                }
            ],
            "muted": false,
            "pinned": false,
            "pleroma": {
                "content": {
                    "text/plain": "FOOD.So yesterday, I embarked on an 8-ish hour adventure of making @JordiGH ’s birthday cake. I used some Japanese methods to make the cake itself, which ended up with an angelfood cake-like texture, except with a very precise amount of egg whipping. I made strawberry jam filling from scratch and maybe should put a little more pectin in it, because it was a bit wet in the end. And buttercream. It was fun and I think it turned out to taste ok, although not very pretty. Next time."
                },
                "conversation_id": 3935046,
                "direct_conversation_id": null,
                "emoji_reactions": [],
                "expires_at": null,
                "in_reply_to_account_acct": null,
                "local": false,
                "parent_visible": false,
                "pinned_at": null,
                "spoiler_text": {
                    "text/plain": ""
                },
                "thread_muted": false
            },
            "poll": null,
            "reblog": null,
            "reblogged": false,
            "reblogs_count": 1,
            "replies_count": 0,
            "sensitive": false,
            "spoiler_text": "",
            "tags": [],
            "text": null,
            "uri": "https://queer.party/users/Not_mermista/statuses/108754223667638011",
            "url": "https://queer.party/@Not_mermista/108754223667638011",
            "visibility": "public"
        },
        "reblogged": false,
        "reblogs_count": 0,
        "replies_count": 0,
        "sensitive": false,
        "spoiler_text": "",
        "tags": [],
        "uri": "https://queer.party/users/Not_mermista/statuses/108754223667638011",
        "url": "https://queer.party/users/Not_mermista/statuses/108754223667638011",
        "visibility": "public"
    }
]
"#;
    fn expected_timeline () -> Timeline {
        let account_fields = [
            AccountField {
                name: "Pronouns".to_string(),
                value: "any".to_string(),
            },
            AccountField {
                name: "Blog".to_string(),
                value: "<a href=\"http://jordi.inversethought.com/about/\">http://jordi.inversethought.com/about/</a>".to_string(),
            },
            AccountField {
                name: "GNU/Linux version".to_string(),
                value: "Debian GNU/Linux".to_string(),
            },
        ].to_vec();

        let account = Account {
            acct: "JordiGH@mathstodon.xyz".to_string(),
            avatar: "https://mathstodon.xyz/system/accounts/avatars/000/013/716/original/c5627ee76d38f8cd.png".to_string(),
            avatar_static: "https://mathstodon.xyz/system/accounts/avatars/000/013/716/original/c5627ee76d38f8cd.png".to_string(),
            bot: false,
            created_at: "2018-05-28T00:34:59.000Z".to_string(),
            display_name: "JordiGH".to_string(),
            fields: account_fields,
            followers_count: 463,
            following_count: 489,
            fqn: "JordiGH@mathstodon.xyz".to_string(),
            id: "823".to_string(),
            locked: true,
            note: "<p>Coder. Mathematician. Hacker-errant.</p><p>You can also email me at jordigh@octave.org</p><p>This year I&#39;m writing a book on mathematics on Twitch, <a href=\"https://twitch.tv/jordigh\"><span>https://</span><span>twitch.tv/jordigh</span><span></span></a></p><p>Please say something to me before following me! I&#39;ll deny your follow request if we haven&#39;t exchanged any words, and I&#39;ll probably approve it if you at least say hi.</p>".to_string(),
            statuses_count: 8260,
            url: "https://mathstodon.xyz/@JordiGH".to_string(),
            username: "JordiGH".to_string()
        };

        let media_attachments = [
            MediaAttachment {
                blurhash: "UNKmzb_00p9I}+Di9$jF%y%LM*sotixZRPjb".to_string(),
                description: "Jordi blowing out his braille candles.".to_string(),
                id: "630770650".to_string(),
                preview_url: "https://content.queer.party/media/media_attachments/files/108/754/216/746/179/434/original/3d697c3985b71698.jpg".to_string(),
                remote_url: "https://content.queer.party/media/media_attachments/files/108/754/216/746/179/434/original/3d697c3985b71698.jpg".to_string(),
                text_url: "https://content.queer.party/media/media_attachments/files/108/754/216/746/179/434/original/3d697c3985b71698.jpg".to_string(),
                r#type: "image".to_string(),
                url: "https://content.queer.party/media/media_attachments/files/108/754/216/746/179/434/original/3d697c3985b71698.jpg".to_string()
            },
        ].to_vec();

        let mentions = [
            Mention {
                acct: "Not_mermista@queer.party".to_string(),
                id: "AGBLDkyQLoL35qrqb2".to_string(),
                url: "https://queer.party/@Not_mermista".to_string(),
                username: "Not_mermista".to_string(),
            },
            Mention {
                acct: "Not_mermista@queer.party".to_string(),
                id: "AGBLDkyQLoL35qrqb2".to_string(),
                url: "https://queer.party/@Not_mermista".to_string(),
                username: "Not_mermista".to_string(),
            },
        ].to_vec();

        let reblog_account = Account {
            acct: "Not_mermista@queer.party".to_string(),
            avatar: "https://content.queer.party/media/accounts/avatars/107/747/955/493/272/175/original/4b23c2d85193e1cb.jpeg".to_string(),
            avatar_static: "https://content.queer.party/media/accounts/avatars/107/747/955/493/272/175/original/4b23c2d85193e1cb.jpeg".to_string(),
            bot: false,
            created_at: "2022-02-05T23:36:00.000Z".to_string(),
            display_name: "Not Mermista".to_string(),
            fields: [
                AccountField {
                    name: "Brain".to_string(),
                    value: "Full of bees".to_string()
                },
            ].to_vec(),
            followers_count: 0,
            following_count: 0,
            fqn: "Not_mermista@queer.party".to_string(),
            id: "AGBLDkyQLoL35qrqb2".to_string(),
            locked: false,
            note: "<p>Nervously joining Mastodon to post cat pictures. ^_^</p>".to_string(),
            statuses_count: 110,
            url: "https://queer.party/@Not_mermista".to_string(),
            username: "Not_mermista".to_string()
        };

        let reblog = Box::new(Status {
            account: reblog_account,
            bookmarked: false,
            content:  "<p>FOOD.<br/>So yesterday, I embarked on an 8-ish hour adventure of making <span class=\"h-card\"><a href=\"https://mathstodon.xyz/@JordiGH\" class=\"u-url mention\">@<span>JordiGH</span></a></span> ’s birthday cake. I used some Japanese methods to make the cake itself, which ended up with an angelfood cake-like texture, except with a very precise amount of egg whipping. I made strawberry jam filling from scratch and maybe should put a little more pectin in it, because it was a bit wet in the end. And buttercream. It was fun and I think it turned out to taste ok, although not very pretty. Next time.</p>".to_string(),
            created_at: "2022-08-02T16:29:18.000Z".to_string(),
            favourited: false,
            favourites_count: 0,
            id: "AM7iej8XAMzhC0GtOK".to_string(),
            in_reply_to_account_id: None,
            in_reply_to_id: None,
            media_attachments: [
                MediaAttachment {
                    blurhash: "UJHxZ$%#OE$|}r4TM_yCOts,ivIUY7%gS4D*".to_string(),
                    description: "Inside crumb of the cake.".to_string(),
                    id: "-339402590".to_string(),
                    preview_url: "https://content.queer.party/media/media_attachments/files/108/754/216/976/293/842/original/c097701db9d26c4f.jpg".to_string(),
                    remote_url: "https://content.queer.party/media/media_attachments/files/108/754/216/976/293/842/original/c097701db9d26c4f.jpg".to_string(),
                    text_url: "https://content.queer.party/media/media_attachments/files/108/754/216/976/293/842/original/c097701db9d26c4f.jpg".to_string(),
                    r#type: "image".to_string(),
                    url: "https://content.queer.party/media/media_attachments/files/108/754/216/976/293/842/original/c097701db9d26c4f.jpg".to_string()
                }
            ].to_vec(),
            mentions: [
                Mention {
                    acct: "JordiGH@mathstodon.xyz".to_string(),
                    id: "823".to_string(),
                    url: "https://mathstodon.xyz/@JordiGH".to_string(),
                    username: "JordiGH".to_string()
                }
            ].to_vec(),
            muted: false,
            pinned: false,
            reblog: None,
            reblogged: false,
            reblogs_count: 1,
            replies_count: 0,
            sensitive: false,
            spoiler_text: "".to_string(),
            uri: "https://queer.party/users/Not_mermista/statuses/108754223667638011".to_string(),
            url: "https://queer.party/@Not_mermista/108754223667638011".to_string(),
            visibility: "public".to_string()
        });

        Timeline {
            statuses: [
                Status {
                    account,
                    bookmarked: false,
                    content: "<p>FOOD.<br/>So yesterday, I embarked on an 8-ish hour adventure of making <span class=\"h-card\"><a href=\"https://mathstodon.xyz/@JordiGH\" class=\"u-url mention\">@<span>JordiGH</span></a></span> ’s birthday cake. I used some Japanese methods to make the cake itself, which ended up with an angelfood cake-like texture, except with a very precise amount of egg whipping. I made strawberry jam filling from scratch and maybe should put a little more pectin in it, because it was a bit wet in the end. And buttercream. It was fun and I think it turned out to taste ok, although not very pretty. Next time.</p>".to_string(),
                    created_at: "2022-08-02T16:52:14.000Z".to_string(),
                    favourited: false,
                    favourites_count: 0,
                    id: "AM7iejUVqeZiIAYReK".to_string(),
                    in_reply_to_account_id: None,
                    in_reply_to_id: None,
                    media_attachments,
                    mentions,
                    muted: false,
                    pinned: false,
                    reblog: Some(reblog),
                    reblogged: false,
                    reblogs_count: 0,
                    replies_count: 0,
                    sensitive: false,
                    spoiler_text: "".to_string(),
                    uri: "https://queer.party/users/Not_mermista/statuses/108754223667638011".to_string(),
                    url: "https://queer.party/users/Not_mermista/statuses/108754223667638011".to_string(),
                    visibility: "public".to_string()
                }
            ].to_vec()
        }
    }

    #[test]
    fn test_parse_pleroma_public_timeline () {
        let got: Timeline = serde_json::from_str(&PLEROMA_JSON_PUBLIC_TIMELINE_FIXTURE).unwrap();
        let expected = expected_timeline();
        assert_eq!(got.statuses[0].reblog, expected.statuses[0].reblog);
        assert_eq!(got, expected)
    }
}
