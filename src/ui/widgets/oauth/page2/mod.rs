use gtk4 as gtk;
use glib::Object;
use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct OauthAssistantPage2(ObjectSubclass<imp::OauthAssistantPage2>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::ConstraintTarget,
                    gtk::Buildable, gtk::Orientable;
}

impl OauthAssistantPage2 {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create oauth assistant page 2")
    }
}
