use gtk4 as gtk;
use glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod page1;
mod page2;
mod page3;

use crate::mastodon::{accounts, oauth};
use crate::db;

pub fn create_oauth_assistant(app: &gtk::Application) -> gtk::Assistant {

    let assistant = gtk::Assistant::builder()
        .application(app)
        .title("Authenticate on your home instance")
        .build();

    let page1 = page1::OauthAssistantPage1::new();
    assistant.append_page(&page1);
    assistant.set_page_type(&page1, gtk::AssistantPageType::Intro);

    let page2 = page2::OauthAssistantPage2::new();
    assistant.append_page(&page2);
    assistant.set_page_type(&page2, gtk::AssistantPageType::Content);

    let page3 = page3::OauthAssistantPage3::new();
    assistant.append_page(&page3);
    assistant.set_page_type(&page3, gtk::AssistantPageType::Summary);

    page1.imp().instance_uri_text_entry.connect_activate(glib::clone!(@weak assistant, @weak page1, @weak page2 => move |_| {
        let instance_fqdn = page1.imp().instance_uri_text_entry.buffer().text();
        let conn = match db::open_db() {
            Ok(db_conn) => db_conn,
            Err(e) => panic!("Error when running the DB migrations: {:?}", e),
        };
        let client = match db::get_oauth_client(&conn, &instance_fqdn) {
            Ok(client) => {
                println!("Found client in DB! {:?}", client);
                client.0
            },
            Err(db::OauthClientDatabaseError::NoClient) => {
                match oauth::register_app(&instance_fqdn, "federatz") {
                    Ok(client) => {
                        db::new_oauth_client(&conn, &client);
                        client
                    },
                    Err(err) => panic!("Error when querying mastodon: {:?}", err),
                }
            }
            Err(err) => panic!("Error when querying the DB: {:?}", err)
        };
        assistant.set_page_complete(&page1, true);
        assistant.next_page();
        let auth_link = oauth::gen_authorize_url(&client);
        let pango_link =
            format!(r#"<a href="{}" title="Generate Token">Click here to generate the authorization Token</a>"#,
                    glib::markup_escape_text(&auth_link));
        page2.imp().authorization_link_label.set_markup(&pango_link);
    }));

    page2.imp().authorization_token_entry.connect_activate(
        glib::clone!(@weak page2, @weak page1, @weak assistant => move |_| {
            let instance_fqdn = page1.imp().instance_uri_text_entry.buffer().text();
            let authorization_token = page2.imp().authorization_token_entry.buffer().text();

            let conn = match db::open_db() {
                Ok(db_conn) => db_conn,
                Err(e) => panic!("Error when running the DB migrations: {:?}", e),
            };

            let client = match db::get_oauth_client(&conn, &instance_fqdn) {
                Ok(client) => {
                    client.0
                },
                Err(err) => {
                    panic!("Error when fetching client from the DB: {:?}", err)
                }
            };

            match oauth::generate_token(&client, &authorization_token) {
                Ok(token) => {
                    match accounts::verify_credentials(&instance_fqdn, token) {
                        Ok(user_account) => {
                            db::new_local_user(&conn, &user_account, &client).unwrap();
                            assistant.set_page_complete(&page2, true);
                            assistant.next_page();
                            println!("{:?}", user_account);
                        },
                        Err(err) => println!("Token seems to be invalid: {:?}", err),
                    }
                },
                Err(err) => println!("Cannot generate the access token: {:?}", err),
            }

        }));

    assistant
}
