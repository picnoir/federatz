use gtk4 as gtk;
use gtk::prelude::*;
use gtk::Application;

mod db;
mod mastodon;
mod ui;

use ui::widgets::oauth::create_oauth_assistant;

fn main() {
    {
        match db::open_db() {
            Ok(mut db_conn) => db::run_migrations(&mut db_conn),
            Err(e) => panic!("Error when running the DB migrations: {:?}", e),
        };
    }
    let app = Application::builder()
        .application_id("fr.alternativebit.federatz")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let oauth_assistant = create_oauth_assistant(app);
    oauth_assistant.present();
}
