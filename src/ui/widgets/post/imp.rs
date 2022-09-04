use std::cell::RefCell;
use gtk4 as gtk;
use gtk::subclass::prelude::*;
use gtk::subclass::window::WindowImpl;
use gtk::prelude::*;
use gtk::{glib, CompositeTemplate, Box, Label};
use glib::subclass::InitializingObject;
use glib::{ParamFlags, ParamSpec, ParamSpecString, Value};
use once_cell::sync::Lazy;

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "./post.ui")]
pub struct Post {
    #[template_child]
    pub post_text_content: TemplateChild<Label>,
    pub content: RefCell<Option<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Post {
    const NAME: &'static str = "Post";
    type Type = super::Post;
    type ParentType = Box;

    fn class_init(c: &mut Self::Class) {
        c.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Post {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(||
                      vec![
                          ParamSpecString::new("content", "Content", "Content", None, ParamFlags::READWRITE)
                      ]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "content" => {
                let content = value.get().expect("Post: missing content property");
                self.content.replace(content);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "content" =>  self.content.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        let label = self.post_text_content.try_get().expect("");
        obj.bind_property("content", &label, "label").build();
    }
}

impl WindowImpl for Post {}
impl WidgetImpl for Post {}
impl BoxImpl for Post {}
impl ApplicationWindowImpl for Post {}
