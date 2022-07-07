use gtk4 as gtk;
use gtk::subclass::prelude::*;
use gtk::subclass::window::WindowImpl;
use gtk::prelude::*;
use gtk::{glib, CompositeTemplate, Box};
use glib::subclass::InitializingObject;

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "./oauthAssistantPage3.ui")]
pub struct OauthAssistantPage3 {
}

#[glib::object_subclass]
impl ObjectSubclass for OauthAssistantPage3 {
    const NAME: &'static str = "oauthAssistantPage3";
    type Type = super::OauthAssistantPage3;
    type ParentType = Box;

    fn class_init(c: &mut Self::Class) {
        c.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for OauthAssistantPage3 {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl WindowImpl for OauthAssistantPage3 {}
impl WidgetImpl for OauthAssistantPage3 {}
impl BoxImpl for OauthAssistantPage3 {}
impl ApplicationWindowImpl for OauthAssistantPage3 {}
