extern crate gtk;

use gtk::prelude::*;

use std::rc::Rc;
pub enum Columns 
{
    Name,
    Description,
    Language,
    Date,
}

pub fn add_columns(_model: &Rc<gtk::ListStore>, treeview: &gtk::TreeView) 
{
    // Column for names
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Name");
        column.add_attribute(&renderer, "text", Columns::Name as i32);
        column.set_sort_column_id(Columns::Name as i32);
        treeview.append_column(&column);
    }

    // Column for description
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Description");
        column.add_attribute(&renderer, "text", Columns::Description as i32);
        column.set_sort_column_id(Columns::Description as i32);
        treeview.append_column(&column);
    }

    // Column for languages
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Language");
        column.add_attribute(&renderer, "text", Columns::Language as i32);
        column.set_sort_column_id(Columns::Language as i32);
        treeview.append_column(&column);
    }

    // Column for date
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Date");
        column.add_attribute(&renderer, "text", Columns::Date as i32);
        column.set_sort_column_id(Columns::Date as i32);
        treeview.append_column(&column);
    }
}
