use std::path::PathBuf;

use dirs;
use rusqlite::{Connection, Error};
use rusqlite_migration::{Migrations, M};

pub fn open_db() -> Result<Connection,Error> {
    let mut data_dir = dirs::data_dir();
    let path = data_dir.get_or_insert(PathBuf::from("./"));
    path.push("federatz");
    std::fs::create_dir_all(&path);
    path.push("federatz.db");
    Connection::open(path)
}

pub fn run_migrations(mut conn: Connection) -> Result<(), rusqlite_migration::Error> {
    let migrations  =
        Migrations::new(vec![
            M::up(r#"
                CREATE TABLE oauth_client(
                    id INTEGER PRIMARY KEY,
                    instance_uri TEXT NOT NULL,
                    client_id TEXT NOT NULL,
                    client_secret TEXT NOT NULL
                );
                CREATE TABLE local_user(
                     token TEXT NOT NULL,
                     username TEXT NOT NULL,
                     display_name TEXT NOT NULL,
                     note TEXT NOT NULL,
                     follower_count INTEGER NOT NULL,
                     following_count INTEGER NOT NULL,
                     statuses_count INTEGER NOT NULL,
                     client INTEGER,
                     FOREIGN KEY(client) REFERENCES oauth_client(id)
                 );
            "#)
    ]);
    migrations.to_latest(&mut conn)
}
