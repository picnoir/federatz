use gtk4 as gtk;
use glib::Object;
use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct Post(ObjectSubclass<imp::Post>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::ConstraintTarget,
                    gtk::Buildable, gtk::Orientable;
}

impl Post {
    pub fn new(content: &String) -> Self {
        Object::new(&[("content", content)]).expect("Failed to create post widget")
    }
}
