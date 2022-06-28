use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

mod oauth;

fn main() {
    let app = Application::builder()
        .application_id("fr.alternativebit.federatz")
        .build();

    let masto_oauth = oauth::register_app("social.alternativebit.fr", "federatz-test");

    println!("{:?}", masto_oauth);

    app.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        let button = Button::with_label("Hello world!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        window.set_child(Some(&button));

        // Show the window.
        window.show();
    });

    app.run();
}
