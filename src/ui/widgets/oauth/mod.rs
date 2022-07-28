use gtk4 as gtk;
use glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod page1;
mod page2;
mod page3;

use crate::mastodon::{accounts, oauth};

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
        let instance_uri = page1.imp().instance_uri_text_entry.buffer().text();
        let ores = oauth::register_app(&instance_uri, "federatz-0.1");
        match ores {
            Ok(res) => {
                assistant.set_page_complete(&page1, true);
                assistant.next_page();
                let auth_link = oauth::gen_authorize_url(&instance_uri, &res);
                let pango_link =
                    format!(r#"<a href="{}" title="Generate Token">Click here to generate the authorization Token</a>"#, glib::markup_escape_text(&auth_link));
                page2.imp().authorization_link_label.set_markup(&pango_link);
            },
            // TODO: read more about Rust error handling, refine error type.
            Err(err) => println!("Error: {:?}", err),
        }
    }));

    page2.imp().authorization_token_entry.connect_activate(
        glib::clone!(@weak page2, @weak page1, @weak assistant => move |_| {
            let instance_uri = page1.imp().instance_uri_text_entry.buffer().text();
            let authorization_token = page2.imp().authorization_token_entry.buffer().text();
            match accounts::verify_credentials(&instance_uri, &authorization_token) {
                Ok(res) => {
                    assistant.set_page_complete(&page2, true);
                    assistant.next_page();
                    // TODO: save to DB
                },
                Err(err) => println!("Error: {:?}", err),

            }
        }));

    assistant
}
