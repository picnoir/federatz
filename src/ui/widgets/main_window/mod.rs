use gtk4 as gtk;
use glib::Object;
use gtk::{glib, gio, Application, ListBox};
use gtk4::prelude::{GtkWindowExt, ActionMapExt};

use crate::ui::widgets::oauth::create_oauth_assistant;
use crate::ui::widgets::post::Post;
use crate::mastodon::timelines;
use crate::db;

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
        action_assistant.connect_activate(|_action, _parameter| {
            let assistant = create_oauth_assistant();
            assistant.present();
        });
        self.add_action(&action_assistant);
    }

    fn init_content(&self, content: &ListBox) {
        let model = gio::ListStore::new(Post::static_type());
        let mut conn = db::open_db().expect("MainWindow: Cannot open the DB!");
        db::run_migrations(&mut conn).expect("MainWindow: cannot run DB migrations!");
        let user = db::get_local_user(&mut conn).expect("MainWindow: cannot get local user!");
        let timeline = timelines::get_personal_timeline(&user.token, "social.alternativebit.fr").expect("MainWindow: cannot fetch user timeline");
        for elem in timeline.statuses {
            content.append(&Post::new(&elem.content));
        }
    }
}
