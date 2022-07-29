use std::path::PathBuf;

use rusqlite::{Connection, named_params};
use rusqlite_migration::{Migrations, M};

use crate::mastodon::oauth::{RegisteredApp, RegisteredAppWithInstanceFqdn, AuthToken};
use crate::mastodon::accounts::{UserAccount, UserAccountWithToken};

#[derive(Debug)]
pub enum OpenDbError {
    RusqliteError(rusqlite::Error),
    IOError(std::io::Error)
}

pub fn open_db() -> Result<Connection,OpenDbError> {
    let mut data_dir = dirs::data_dir();
    let path = data_dir.get_or_insert(PathBuf::from("./"));
    path.push("federatz");
    std::fs::create_dir_all(&path).map_err(|err| OpenDbError::IOError(err))?;
    path.push("federatz.db");
    Connection::open(path).map_err(|err| OpenDbError::RusqliteError(err))
}

pub fn run_migrations(conn: &mut Connection) -> Result<(), rusqlite_migration::Error> {
    let migrations  =
        Migrations::new(vec![
            M::up(r#"
                CREATE TABLE oauth_client(
                    instance_fqdn TEXT NOT NULL,
                    client_id TEXT NOT NULL,
                    client_secret TEXT NOT NULL,
                    name TEXT NOT NULL,
                    redirect_uri TEXT NOT NULL,
                    website TEXT
                );
                CREATE TABLE local_user(
                     id INTEGER,
                     username TEXT NOT NULL,
                     display_name TEXT NOT NULL,
                     locked BOOLEAN NOT NULL,
                     bot BOOLEAN NOT NULL,
                     created_at TEXT NOT NULL,
                     note TEXT NOT NULL,
                     url TEXT NOT NULL,
                     avatar TEXT NOT NULL,
                     avatar_static TEXT NOT NULL,
                     header TEXT NOT NULL,
                     header_static TEXT NOT NULL,
                     followers_count INTEGER NOT NULL,
                     following_count INTEGER NOT NULL,
                     statuses_count INTEGER NOT NULL,
                     token_access_token TEXT NOT NULL,
                     token_created_at INTEGER NOT NULL,
                     token_expires_in INTEGER NOT NULL,
                     token_id INTEGER NOT NULL,
                     token_me TEXT NOT NULL,
                     token_refresh_token TEXT NOT NULL,
                     token_scope TEXT NOT NULL,
                     token_token_type TEXT NOT NULL,
                     client INTEGER,
                     FOREIGN KEY(client) REFERENCES oauth_client(id)
                );

            "#)
    ]);
    migrations.to_latest(conn)
}

#[derive(Debug, PartialEq)]
pub enum OauthClientDatabaseError {
    NoClient,
    RusqliteError(rusqlite::Error)
}

impl From<rusqlite::Error> for OauthClientDatabaseError {
    fn from(error: rusqlite::Error) -> Self {
        OauthClientDatabaseError::RusqliteError(error)
    }
}
pub fn new_oauth_client(conn: &Connection, client: &RegisteredAppWithInstanceFqdn) -> Result<i64,rusqlite::Error> {
    let mut stmt =
        conn.prepare(r#"
            INSERT INTO oauth_client
                (instance_fqdn, client_id, client_secret, name, redirect_uri, website)
                VALUES (:instance_fqdn, :client_id, :client_secret, :name, :redirect_uri, :website)"#
        )?;
    stmt.insert(
        named_params! {
            ":instance_fqdn": client.instance_fqdn,
            ":client_id": client.registered_app.client_id,
            ":client_secret": client.registered_app.client_secret,
            ":name": client.registered_app.name,
            ":redirect_uri": client.registered_app.redirect_uri,
            ":website": client.registered_app.website,
        }
    )
}

pub fn delete_oauth_client(conn: &Connection, client: &RegisteredAppWithInstanceFqdn) -> Result<usize,rusqlite::Error> {
    let mut stmt =
        conn.prepare(r#"
            DELETE FROM oauth_client
                WHERE client_id = :client_id AND client_secret = :client_secret"#
        )?;
    stmt.execute(
        named_params! {
            ":client_id": client.registered_app.client_id,
            ":client_secret": client.registered_app.client_secret
        }
    )
}

pub fn get_oauth_client(conn: &Connection, instance_fqdn: &str) -> Result<(RegisteredAppWithInstanceFqdn, i64), OauthClientDatabaseError> {
    let mut stmt =
        conn.prepare(
            r#"SELECT ROWID, instance_fqdn, client_id, client_secret, name, redirect_uri, website
                FROM oauth_client
                WHERE instance_fqdn = :instance_fqdn"#)?;
    let mut rows = stmt.query_and_then(
        named_params! {
            ":instance_fqdn": instance_fqdn
        },
        |row| {
            let registered_app = RegisteredApp {
                client_id: row.get("client_id")?,
                client_secret: row.get("client_secret")?,
                name: row.get("name")?,
                redirect_uri: row.get("redirect_uri")?,
                website: row.get("website")?,
            };
            Ok((RegisteredAppWithInstanceFqdn {
                instance_fqdn: row.get("instance_fqdn")?,
                registered_app
            }, row.get("ROWID")?))})?;
    rows.next().ok_or(OauthClientDatabaseError::NoClient)?
}

#[derive(Debug, PartialEq)]
pub enum GetLocalUserDatabaseError {
    NoUser,
    ManyUsers,
    RusqliteError(rusqlite::Error)
}

impl From<rusqlite::Error> for GetLocalUserDatabaseError {
    fn from(error: rusqlite::Error) -> Self {
        GetLocalUserDatabaseError::RusqliteError(error)
    }
}

#[derive(Debug, PartialEq)]
pub enum NewLocalUserDatabaseError {
    ClientError(OauthClientDatabaseError),
    RusqliteError(rusqlite::Error)
}

impl From<rusqlite::Error> for NewLocalUserDatabaseError {
    fn from(error: rusqlite::Error) -> Self {
        NewLocalUserDatabaseError::RusqliteError(error)
    }
}

impl From<OauthClientDatabaseError> for NewLocalUserDatabaseError {
    fn from(error:OauthClientDatabaseError) -> Self {
        NewLocalUserDatabaseError::ClientError(error)
    }
}

/// Creates a new user account and returns its Sqlite ROWID.
pub fn new_local_user(conn: &Connection, user: &UserAccountWithToken, client: &RegisteredAppWithInstanceFqdn) -> Result<i64, NewLocalUserDatabaseError> {
    let mut stmt =
        conn.prepare(r#"
            INSERT INTO local_user (id, username, display_name, locked, bot, created_at, note, url, avatar,
                                    avatar_static, header, header_static, followers_count, following_count, statuses_count,
                                    token_access_token, token_created_at, token_expires_in, token_id, token_me, token_refresh_token,
                                    token_scope, token_token_type, client)
            VALUES (:id, :username, :display_name, :locked, :bot, :created_at, :note, :url, :avatar,
                    :avatar_static, :header, :header_static, :followers_count, :following_count, :statuses_count,
                    :token_access_token, :token_created_at, :token_expires_in, :token_id, :token_me, :token_refresh_token,
                    :token_scope, :token_token_type, :client)"#)?;
    let client = get_oauth_client(&conn, &client.instance_fqdn)?;
    let new_user = stmt.insert(
        named_params! {
            ":id": user.user_account.id,
            ":username": user.user_account.username,
            ":display_name": user.user_account.display_name,
            ":locked": user.user_account.locked,
            ":bot": user.user_account.bot,
            ":created_at": user.user_account.created_at,
            ":note": user.user_account.note,
            ":url": user.user_account.url,
            ":avatar": user.user_account.avatar,
            ":avatar_static": user.user_account.avatar_static,
            ":header": user.user_account.header,
            ":header_static": user.user_account.header_static,
            ":followers_count": user.user_account.followers_count,
            ":following_count": user.user_account.following_count,
            ":statuses_count": user.user_account.statuses_count,
            ":token_access_token": user.token.access_token,
            ":token_created_at": user.token.created_at,
            ":token_expires_in": user.token.expires_in,
            ":token_id": user.token.id,
            ":token_me": user.token.me,
            ":token_refresh_token": user.token.refresh_token,
            ":token_scope": user.token.scope,
            ":token_token_type": user.token.token_type,
            ":client": client.1})?;
    Ok(new_user)
}

pub fn get_local_user(conn: &Connection) -> Result<UserAccountWithToken, GetLocalUserDatabaseError> {
    let mut stmt =
        conn.prepare(r#"
            SELECT CAST(id AS TEXT) as id, username, display_name, locked, bot, created_at, note, url, avatar,
                   avatar_static, header, header_static, followers_count, following_count,
                   statuses_count, token_access_token, token_created_at, token_expires_in, token_id, token_me, token_refresh_token, token_scope, token_token_type FROM local_user"#).map_err(|err| GetLocalUserDatabaseError::RusqliteError(err))?;
    let mut rows = stmt.query_and_then(
        [],
        |row| {
            let user_account =  UserAccount {
                id: row.get("id")?,
                username: row.get("username")?,
                display_name: row.get("display_name")?,
                locked: row.get("locked")?,
                bot: row.get("bot")?,
                created_at: row.get("created_at")?,
                note: row.get("note")?,
                url: row.get("url")?,
                avatar: row.get("avatar")?,
                avatar_static: row.get("avatar_static")?,
                header: row.get("header")?,
                header_static: row.get("header_static")?,
                followers_count: row.get("followers_count")?,
                following_count: row.get("following_count")?,
                statuses_count: row.get("statuses_count")?,
            };
            let token = AuthToken {
                access_token: row.get("token_access_token")?,
                created_at: row.get("token_created_at")?,
                expires_in: row.get("token_expires_in")?,
                id: row.get("token_id")?,
                me: row.get("token_me")?,
                refresh_token: row.get("token_refresh_token")?,
                scope: row.get("token_scope")?,
                token_type: row.get("token_token_type")?
            };
            Ok(UserAccountWithToken {
                token,
                user_account
            })}).map_err(|err| GetLocalUserDatabaseError::RusqliteError(err))?;


    let user = rows.next().ok_or(GetLocalUserDatabaseError::NoUser)?;
    if rows.next().is_none() {
        user
    } else {
        Err(GetLocalUserDatabaseError::ManyUsers)
    }
}

pub fn delete_local_user(conn: &Connection, user: &UserAccount) -> Result<usize, rusqlite::Error> {
    let mut stmt =
        conn.prepare("DELETE FROM local_user WHERE username = :username")?;
    stmt.execute(named_params! { ":username": user.username })
}

#[cfg(test)]
mod test {
    use crate::db::{UserAccount, UserAccountWithToken, RegisteredApp, RegisteredAppWithInstanceFqdn, AuthToken};

    fn dummy_client () -> RegisteredAppWithInstanceFqdn {
        let registered_app = RegisteredApp {
            client_id: String::from("dummy-client-id"),
            client_secret: String::from("dummy-client-secret"),
            name: String::from("dummy-name"),
            redirect_uri: String::from("dummy-redirect-uri"),
            website: None,
        };
        RegisteredAppWithInstanceFqdn {
            registered_app,
            instance_fqdn: String::from("dummy.instance"),
        }
    }

    fn dummy_local_user (id: &str) -> UserAccountWithToken {
        let user_account = UserAccount {
            id: String::from(id),
            username: String::from(format!("Ninjatrappeur{}", id)),
            display_name: String::from("⠴Ninjatrappeur⠦"),
            locked: false,
            bot: false,
            created_at: String::from("2018-05-26T13:07:04.000Z"),
            note: String::from("dummy-note"),
            url: String::from("https://social.alternativebit.fr/users/Ninjatrappeur"),
            avatar: String::from("https://social.alternativebit.fr/media/3f9ad2d6f473c954506f404de5a3636905035f32ec9f1f07deca0a72e88ac3e8.blob"),
            avatar_static: String::from("https://social.alternativebit.fr/media/3f9ad2d6f473c954506f404de5a3636905035f32ec9f1f07deca0a72e88ac3e8.blob"),
            header: String::from("https://social.alternativebit.fr/media/b5f2cf7e-7892-4692-b0c8-157818ad1b87/57CF8FB0FE2143FF1267A90BD3217D8D433FA2381C56E36264352A3EA0F17838.png"),
            header_static: String::from("https://social.alternativebit.fr/media/b5f2cf7e-7892-4692-b0c8-157818ad1b87/57CF8FB0FE2143FF1267A90BD3217D8D433FA2381C56E36264352A3EA0F17838.png"),
            followers_count: 615,
            following_count: 191,
            statuses_count: 1297,
        };
        let token = AuthToken {
            access_token: "dummy-access-token".to_string(),
            created_at: 42,
            expires_in: 43,
            id: 44,
            me: "mario".to_string(),
            refresh_token: "refresh token".to_string(),
            scope: "read write push".to_string(),
            token_type: "Bearer".to_string(),
        };
        UserAccountWithToken {
            token,
            user_account
        }
    }

    fn setup_temp_db () -> rusqlite::Connection {
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();
        super::run_migrations(&mut conn).unwrap();
        conn
    }

    #[test]
    fn new_oauth_client () {
        let conn = setup_temp_db();
        let dummy_client = dummy_client();
        super::new_oauth_client(&conn, &dummy_client).unwrap();
    }

    #[test]
    fn get_oauth_client () {
        let conn = setup_temp_db();
        let dummy_client = dummy_client();
        super::new_oauth_client(&conn, &dummy_client).unwrap();
        assert_eq!(super::get_oauth_client(&conn, &dummy_client.instance_fqdn).unwrap().0, dummy_client);
    }

    #[test]
    fn new_local_user () {
        let conn = setup_temp_db();
        let dummy_client = dummy_client();
        let dummy_local_user = dummy_local_user("1");
        super::new_oauth_client(&conn, &dummy_client).unwrap();
        super::new_local_user(&conn, &dummy_local_user, &dummy_client).unwrap();
    }

    #[test]
    fn get_local_user_ok () {
        let conn = setup_temp_db();
        let dummy_client = dummy_client();
        let dummy_local_user = dummy_local_user("1");
        super::new_oauth_client(&conn, &dummy_client).unwrap();
        super::new_local_user(&conn, &dummy_local_user, &dummy_client).unwrap();
        assert_eq!(super::get_local_user(&conn), Ok(dummy_local_user));
    }

    #[test]
    fn get_local_user_fail_many () {
        let conn = setup_temp_db();
        let dummy_client = dummy_client();
        let dummy_local_user_1 = dummy_local_user("1");
        let dummy_local_user_2 = dummy_local_user("2");
        super::new_oauth_client(&conn, &dummy_client).unwrap();
        super::new_local_user(&conn, &dummy_local_user_1, &dummy_client).unwrap();
        super::new_local_user(&conn, &dummy_local_user_2, &dummy_client).unwrap();
        assert_eq!(super::get_local_user(&conn), Err(super::GetLocalUserDatabaseError::ManyUsers));
    }

    #[test]
    fn get_local_user_fail_none () {
        let conn = setup_temp_db();
        assert_eq!(super::get_local_user(&conn), Err(super::GetLocalUserDatabaseError::NoUser));
    }
}
