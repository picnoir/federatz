use gtk4 as gtk;
use gtk::prelude::*;
use gtk::Application;

mod oauth;
mod ui;

use ui::widgets::oauth::create_oauth_assistant;

fn main() {
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
