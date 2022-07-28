use gtk4 as gtk;
use gtk::subclass::prelude::*;
use gtk::subclass::window::WindowImpl;
use gtk::prelude::*;
use gtk::{glib, CompositeTemplate, Box, Label, Entry};
use glib::subclass::InitializingObject;

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "./oauthAssistantPage2.ui")]
pub struct OauthAssistantPage2 {
    #[template_child]
    pub authorization_link_label: TemplateChild<Label>,
    #[template_child]
    pub authorization_token_entry: TemplateChild<Entry>,
}

#[glib::object_subclass]
impl ObjectSubclass for OauthAssistantPage2 {
    const NAME: &'static str = "oauthAssistantPage2";
    type Type = super::OauthAssistantPage2;
    type ParentType = Box;

    fn class_init(c: &mut Self::Class) {
        c.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for OauthAssistantPage2 {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl WindowImpl for OauthAssistantPage2 {}
impl WidgetImpl for OauthAssistantPage2 {}
impl BoxImpl for OauthAssistantPage2 {}
impl ApplicationWindowImpl for OauthAssistantPage2 {}
