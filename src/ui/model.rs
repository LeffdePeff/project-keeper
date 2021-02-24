extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

pub struct Project 
{
    name: String,
    description: String,
    language: String,
    date: String,
}

pub static mut PROJECTS: Vec<Project> = vec![];

pub fn create_model() -> gtk::ListStore 
{
    let col_types: [glib::Type; 6] = [
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::U32,
        glib::Type::Bool,
    ];

    let store = gtk::ListStore::new(&col_types);

    let col_indices: [u32; 6] = [0, 1, 2, 3, 4, 5];

    unsafe {
        PROJECTS = vec![
            Project {name: "Climeat".to_string(), description: "Food Stuff".to_string(), language: "Dart".to_string(), date: "now".to_string()},
            Project {name: "Big Power CLI".to_string(), description: "GMP based big power calculator".to_string(), language: "Rust".to_string(), date: "now".to_string()},
            Project {name: "Big Power GUI".to_string(), description: "gui version of Big Power CLI".to_string(), language: "Rust".to_string(), date: "now".to_string()},
            Project {name: "Cube Flight".to_string(), description: "Game".to_string(), language: "Godot".to_string(), date: "now".to_string()},
        ];

        for (_d_idx, d) in PROJECTS.iter().enumerate() {
            let values: [&dyn ToValue; 6] = [
                &d.name,
                &d.description,
                &d.language,
                &d.date,
                &0u32,
                &false,
            ];
            store.set(&store.append(), &col_indices, &values);
        }
        store
    }
}

