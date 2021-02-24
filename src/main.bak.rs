extern crate gtk;
extern crate gio;

use std::borrow::Borrow;

use gtk::prelude::*;
use gio::prelude::*;

struct Project
{
    name: String,
    description: String,
    language: String,
}

fn main() 
{
    let projects = [
        Project {name: "project keeper".to_string(), description: "keep projects".to_string(), language: "Rust".to_string()},
        Project {name: "climeat".to_string(), description: "climeat app".to_string(), language: "Flutter".to_string()},
        Project {name: "Big Power".to_string(), description: "calculate big powers".to_string(), language: "Rust".to_string()},
        Project {name: "Big Power GUI".to_string(), description: "gui version of Big Power".to_string(), language: "Rust".to_string()},
    ];

    let application = gtk::Application::new(
        Some("com.leffdepeff.project_keeper"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate( move |app| {
        let glade_src = include_str!("../gtk_layout.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let window: gtk::Window = builder.get_object("application_window").unwrap();

        let project_glade_src = include_str!("../projects_layout.glade");
        let project_builder = gtk::Builder::from_string(project_glade_src);
        let mut project_widgets: Vec<gtk::Grid> = vec![];

        window.set_default_size(350, 200);
        window.set_application(Some(app));
        window.set_title("Project Keeper");

        let list_box: gtk::ListBox = builder.get_object("list_box").unwrap();

        for x in projects.iter() {
            let grid: gtk::Grid = project_builder.get_object("grid").unwrap();
            let name: gtk::Label = project_builder.get_object("name").unwrap();
            let description: gtk::Label = project_builder.get_object("description").unwrap();
            let language: gtk::Label = project_builder.get_object("language").unwrap();

            name.set_text(&x.name);
            description.set_text(&x.description);
            language.set_text(&x.language);
            project_widgets.push(grid.clone());
        }

        let mut count =0;
        while count < project_widgets.len() {
            list_box.add(project_widgets[count].borrow());
            count += 1;
        }

        window.show_all();
    });

    application.run(&[]);
}