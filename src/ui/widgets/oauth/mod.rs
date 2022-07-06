use gtk4 as gtk;
use gtk::glib;

mod page1;

pub fn create_oauth_assistant(app: &gtk::Application) -> gtk::Assistant {
    let assistant = gtk::Assistant::builder()
        .application(app)
        .title("Authenticate on your home instance")
        .build();
    let page1 = page1::OauthAssistantPage1::new();
    assistant.append_page(&page1);
    assistant.set_page_type(&page1, gtk::AssistantPageType::Intro);
    assistant
}
