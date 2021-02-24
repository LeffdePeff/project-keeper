extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::borrow::Borrow;
use std::rc::Rc;

#[path = "columns.rs"] mod columns;

pub fn build_ui(application: &gtk::Application) 
{
    let glade_src = include_str!("glade/gtk_layout.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window:gtk::ApplicationWindow = gtk::ApplicationWindow::new(application);
    let notebook: gtk::Notebook = builder.get_object("notebook").unwrap();

    let glade_src_new = include_str!("glade/new_project.glade");
    let builder_new = gtk::Builder::from_string(glade_src_new);
    let new_project_cancel: gtk::Button = builder_new.get_object("cancel_button").unwrap();

    window.set_title("Project Keeper");
    window.set_border_width(10);
    window.set_default_size(640, 450);

    let new_project_button: gtk::Button = builder.get_object("new_project_button").unwrap();

    window.add(notebook.borrow());

    let notebook_new = notebook.clone();

    new_project_button.connect_clicked(move |_| {
        let new_project_box: gtk::Box = builder_new.get_object("new_project_box").unwrap();
        
        notebook_new.append_page(new_project_box.borrow(), Some(gtk::Label::new(None).borrow()));
        notebook_new.set_current_page(Some(1));
    });

    let notebook_cancel = notebook.clone();

    new_project_cancel.connect_clicked(move |_| {
        notebook_cancel.remove_page(Some(1));
        notebook_cancel.set_current_page(Some(0));
    });

    let sw: gtk::ScrolledWindow = builder.get_object("sw").unwrap();

    let model = Rc::new(create_model());
    let treeview = gtk::TreeView::with_model(&*model);
    treeview.set_vexpand(true);
    treeview.set_search_column(columns::Columns::Description as i32);

    sw.add(&treeview);

    columns::add_columns(&model, &treeview);

    window.show_all();
}

struct Project 
{
    name: String,
    description: String,
    language: String,
    date: String,
}

fn create_model() -> gtk::ListStore 
{
    let col_types: [glib::Type; 6] = [
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::U32,
        glib::Type::Bool,
    ];

    let projects: Vec<Project> = vec![
        Project {name: "Climeat".to_string(), description: "Food Stuff".to_string(), language: "Dart".to_string(), date: "now".to_string()},
        Project {name: "Big Power CLI".to_string(), description: "GMP based big power calculator".to_string(), language: "Rust".to_string(), date: "now".to_string()},
        Project {name: "Big Power GUI".to_string(), description: "gui version of Big Power CLI".to_string(), language: "Rust".to_string(), date: "now".to_string()},
        Project {name: "Cube Flight".to_string(), description: "Game".to_string(), language: "Godot".to_string(), date: "now".to_string()},
    ];

    let store = gtk::ListStore::new(&col_types);

    let col_indices: [u32; 6] = [0, 1, 2, 3, 4, 5];

    for (_d_idx, d) in projects.iter().enumerate() {
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

