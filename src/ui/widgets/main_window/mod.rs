use gtk4 as gtk;
use glib::Object;
use gtk::{glib, gio, Application};
use gtk4::prelude::{GtkWindowExt, ActionMapExt};

use crate::ui::widgets::oauth::create_oauth_assistant;

mod imp;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gtk::Accessible, gtk::ConstraintTarget,
                    gtk::Buildable, gtk::Native, gtk::Root,
                    gtk::ShortcutManager, gio::ActionGroup,
                    gio::ActionMap;
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        Object::new(&[("application", app)]).expect("Failed to create the main window")
    }

    fn setup_actions(&self) {
        let action_assistant = gio::SimpleAction::new("open_assistant", None);
        action_assistant.connect_activate(|action, parameter| {
            let assistant = create_oauth_assistant();
            assistant.present();
        });
        self.add_action(&action_assistant);
    }
}
