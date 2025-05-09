use gtk::{prelude::*, EventControllerKey};
use std::ffi::CStr;

use crate::logic::entries::IndexedEntry;

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


pub fn list_box(indexed_entries: Vec<IndexedEntry>, search_query: String, input_bar: &gtk::Entry, app: &gtk::Application) -> gtk::ListBox {
    let list_box = gtk::ListBox::builder()
        .css_name("list-box")
        .selection_mode(gtk::SelectionMode::Single)
        .build();
    
    populate_list_box(&list_box, indexed_entries.clone(), search_query);
    
    let list_box_weak = list_box.downgrade();
    
    let indexed_entries = indexed_entries.clone();
    
    input_bar.connect_changed(move |entry| {
        if let Some(list_box) = list_box_weak.upgrade() {
            let query = entry.text();
            println!("query: {}", query);
            populate_list_box(&list_box, indexed_entries.clone(), query.to_string());
        }
    });

    let list_box_weak_activate = list_box.downgrade();
    let app = app.clone();
    input_bar.connect_activate(move |_entry| {
        if let Some(list_box) = list_box_weak_activate.upgrade() {
            if let Some(first_app) = list_box.first_child() {
                first_app.activate();
                unsafe {
                    if let Some(entry_data) = first_app.data::<IndexedEntry>("entry_data") {
                        let entry_data = entry_data.as_ref();
                        let name = CStr::from_ptr(entry_data.entry.name).to_string_lossy();
                        let value = CStr::from_ptr(entry_data.entry.value).to_string_lossy();
                        println!("Selected entry: {} with value: {}", name, value);
                        if (entry_data.plugin.handle_selection)(entry_data.entry.value) {
                            app.quit();
                        } else {
                            println!("Failed to open application");
                        }
                    }
                }
            }
        }
    });

    let enter_controller = EventControllerKey::new();
    enter_controller.connect_key_pressed(move |_, key, _, _| {
        println!("key: {}", key);
        gtk::glib::Propagation::Proceed
    });
    input_bar.add_controller(enter_controller);
    list_box
}

#[allow(dead_code)]
pub fn populate_list_box(
    list_box: &gtk::ListBox,
    indexed_entries: Vec<IndexedEntry>,
    search_query: String,
) {
    while let Some(row) = list_box.last_child() {
        list_box.remove(&row);
    }
    let query = search_query.clone().to_lowercase();
    let mut count = 0;
    let one_word = query.split(" ").count() == 1;
    let prefix = query.split(" ").next().unwrap_or("");
    let rest = query.split(" ").skip(1).collect::<Vec<&str>>().join(" ");

    for indexed_entry in indexed_entries {
        if count == 12 {
            break;
        }
        let default_prefix =
            unsafe { CStr::from_ptr(indexed_entry.plugin.info.default_prefix).to_string_lossy() }
                .to_lowercase();
        let name = unsafe { CStr::from_ptr(indexed_entry.entry.name).to_string_lossy() };
        let name_lower = name.to_lowercase();

        // Simplified matching logic
        let matches = if query.is_empty() {
            true
        } else if default_prefix.is_empty() {
            name_lower.contains(&query)
        } else if one_word {
            default_prefix.starts_with(prefix)
        } else {
            default_prefix == prefix && name_lower.starts_with(&rest)
        };

        if matches {
            let row = gtk::ListBoxRow::new();
            // Store the IndexedEntry with the row
            unsafe {
                row.set_data("entry_data", indexed_entry.clone());
            }
            
            let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 10);
            if !indexed_entry.entry.icon.is_null() {
                let icon_name =
                    unsafe { CStr::from_ptr(indexed_entry.entry.icon).to_string_lossy() };
                let image = gtk::Image::from_icon_name(&icon_name);
                image.set_pixel_size(24);
                hbox.append(&image);
            }
            let label = gtk::Label::new(Some(&name));
            hbox.append(&label);
            row.set_child(Some(&hbox));
            list_box.append(&row);
            count += 1;
        }
    }
}
