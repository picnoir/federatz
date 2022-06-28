use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Orientation};

use std::cell::{RefCell};
use std::rc::Rc;

mod oauth;

fn main() {
    let app = Application::builder()
        .application_id("fr.alternativebit.federatz")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();
        let vert_box = Box::new(Orientation::Vertical, 10);
        let oauth_registered_app = Rc::new(RefCell::new(None));
        let cloned_oauth_registered_app = oauth_registered_app.clone();

        let button_reg = Button::with_label("Register App");
        button_reg.connect_clicked(move |_| {
            let ap_reg = oauth::register_app("social.alternativebit.fr", "federatz-test");
            match ap_reg {
                Ok(ap_reg) => {
                    {
                        oauth_registered_app.replace(Some(ap_reg.clone()));
                        println!("{:?}", ap_reg.clone());
                    }
                },
                Err(_) => ()
            }
        });

        let button_auth = Button::with_label("Authorize App");
        button_auth.connect_clicked(move |_| {
            let orab = cloned_oauth_registered_app.borrow().clone();
            match orab {
                Some(orab) => {
                    let url = oauth::gen_authorize_url(
                        "social.alternativebit.fr",
                        &orab);
                    println!("{}", url);
                },
                None => ()
            }
        });
        vert_box.append(&button_reg);
        vert_box.append(&button_auth);
        window.set_child(Some(&vert_box));

        // Show the window.
        window.show();
    });

    app.run();
}
