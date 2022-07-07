use gtk4 as gtk;

mod page1;
mod page2;
mod page3;

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

    assistant.set_page_complete(&page1, true);

    assistant
}
