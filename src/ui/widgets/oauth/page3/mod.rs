use gtk4 as gtk;
use glib::Object;
use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct OauthAssistantPage3(ObjectSubclass<imp::OauthAssistantPage3>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::ConstraintTarget,
                    gtk::Buildable, gtk::Orientable;
}

impl OauthAssistantPage3 {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create oauth assistant page 3")
    }
}
