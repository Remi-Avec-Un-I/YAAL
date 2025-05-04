use gtk::prelude::*;
use std::ffi::CStr;

use crate::{loader::loader, logic::entries::{bind_entries, IndexedEntry}};

pub fn vbox() -> gtk::Box {
    gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(0)
        .build()
}

pub fn input_bar() -> gtk::Entry {
    gtk::Entry::builder()
        .placeholder_text("Type your message here...")
        .build()
}

pub fn list_box(indexed_entries: Vec<IndexedEntry>) -> gtk::ListBox {
    let list_box = gtk::ListBox::builder()
        .css_name("list-box")
        .selection_mode(gtk::SelectionMode::Single)
        .build();
    for entry in indexed_entries {
        let row = gtk::ListBoxRow::new();
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 10); 
        if !entry.entry.icon.is_null() {
            let icon_name = unsafe { CStr::from_ptr(entry.entry.icon).to_string_lossy() };
            let image = gtk::Image::from_icon_name(&icon_name);
            image.set_pixel_size(24);
            hbox.append(&image);
        }
        let name = unsafe { CStr::from_ptr(entry.entry.name).to_string_lossy() };
        let label = gtk::Label::new(Some(&name));
        hbox.append(&label);
        row.set_child(Some(&hbox));
        list_box.append(&row);
    }
    list_box
}
