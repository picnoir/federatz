use gtk4 as gtk;
use gtk::subclass::prelude::*;
use gtk::subclass::window::WindowImpl;
use gtk::prelude::*;
use gtk::{glib, CompositeTemplate, Label, Box, Entry};
use glib::subclass::InitializingObject;

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "page1.ui")]
pub struct OauthAssistantPage1 {
    #[template_child]
    pub instance_uri_text_entry: TemplateChild<Entry>,
    #[template_child]
    pub instance_uri_label: TemplateChild<Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for OauthAssistantPage1 {
    const NAME: &'static str = "oauthAssistantPage1";
    type Type = super::OauthAssistantPage1;
    type ParentType = Box;

    fn class_init(c: &mut Self::Class) {
        c.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for OauthAssistantPage1 {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl WindowImpl for OauthAssistantPage1 {}
impl WidgetImpl for OauthAssistantPage1 {}
impl BoxImpl for OauthAssistantPage1 {}
impl ApplicationWindowImpl for OauthAssistantPage1 {}
