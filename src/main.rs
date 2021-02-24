extern crate gio;

#[path = "ui/user_interface.rs"] mod user_interface;

use gio::prelude::*;

use std::env::args;

fn main() {
    let application = gtk::Application::new(
        Some("com.leffdepeff.project_keeper"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_startup(|app| {
        user_interface::build_ui(app);
    });

    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}