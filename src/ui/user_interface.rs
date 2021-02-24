extern crate gio;
extern crate gtk;

use gtk::prelude::*;

use std::borrow::Borrow;
use std::rc::Rc;

#[path = "columns.rs"] mod columns;
#[path = "model.rs"] mod model;

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

    let model = Rc::new(model::create_model());
    let treeview = gtk::TreeView::with_model(&*model);
    treeview.set_vexpand(true);
    treeview.set_search_column(columns::Columns::Description as i32);

    sw.add(&treeview);

    columns::add_columns(&model, &treeview);

    window.show_all();
}