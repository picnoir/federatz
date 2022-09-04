use gtk4 as gtk;
use gtk::subclass::prelude::*;
use gtk::subclass::window::WindowImpl;
use gtk::prelude::*;
use gtk::{glib, CompositeTemplate, ListBox};
use glib::subclass::InitializingObject;

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "./main-window.ui")]
pub struct MainWindow {
    #[template_child]
    content: TemplateChild<ListBox>,
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(c: &mut Self::Class) {
        c.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindow {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        obj.setup_actions();
        let content: ListBox = self.content
            .try_get()
            .expect("Main window: content should be instantiated.");
        obj.init_content(&content);
    }
}

impl WidgetImpl for MainWindow {}
impl WindowImpl for MainWindow {}
impl ApplicationWindowImpl for MainWindow {}
