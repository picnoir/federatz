use gtk4 as gtk;
use glib::Object;
use gtk::{gio, glib, Application};

mod imp;

glib::wrapper! {
    pub struct OauthAssistantPage1(ObjectSubclass<imp::OauthAssistantPage1>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::ConstraintTarget,
                    gtk::Buildable, gtk::Orientable;
}

impl OauthAssistantPage1 {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create oauth assistant page 1")
    }
}
