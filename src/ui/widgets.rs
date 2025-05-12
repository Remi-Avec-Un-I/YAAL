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
    let app_activate = app.clone();
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
                            app_activate.quit();
                        } else {
                            println!("Failed to open application");
                        }
                    }
                }
            }
        }
    });

    // Add double-click handler
    let app_double_click = app.clone();
    let list_box_double_click = list_box.clone();
    let gesture = gtk::GestureClick::new();
    gesture.set_button(gdk::BUTTON_PRIMARY);
    gesture.connect_pressed(move |_, n_press, _: f64, _| {
        if n_press == 2 {
            if let Some(row) = list_box_double_click.selected_row() {
                unsafe {
                    if let Some(entry_data) = row.data::<IndexedEntry>("entry_data") {
                        let entry_data = entry_data.as_ref();
                        let name = CStr::from_ptr(entry_data.entry.name).to_string_lossy();
                        let value = CStr::from_ptr(entry_data.entry.value).to_string_lossy();
                        println!("Double-clicked entry: {} with value: {}", name, value);
                        if (entry_data.plugin.handle_selection)(entry_data.entry.value) {
                            app_double_click.quit();
                        }
                    }
                }
            }
        }
    });
    list_box.add_controller(gesture);
    let list_box_key_event = list_box.clone();
    let app_list_box_key_event = app.clone();
    let input_bar_key_event = input_bar.clone();
    let key_enter_controller = EventControllerKey::new();
    key_enter_controller.connect_key_pressed(move |_, key, _, _| {
        if key == gdk::Key::Return {
            if let Some(widget) = list_box_key_event.focus_child() {
                if let Some(row) = widget.downcast_ref::<gtk::ListBoxRow>() {
                    row.activate();
                    unsafe {
                        if let Some(entry_data) = row.data::<IndexedEntry>("entry_data") {
                            let entry_data = entry_data.as_ref();
                            let name = CStr::from_ptr(entry_data.entry.name).to_string_lossy();
                            let value = CStr::from_ptr(entry_data.entry.value).to_string_lossy();
                            println!("Selected entry: {} with value: {}", name, value);
                            if (entry_data.plugin.handle_selection)(entry_data.entry.value) {
                                app_list_box_key_event.quit();
                            }
                        }
                    }
                }
            }
        }
        if key == gdk::Key::Escape {
            app_list_box_key_event.quit();
        } else if let Some(unicode) = key.to_unicode() {
            println!("key unicode: {}", key);
            if unicode.is_ascii_graphic() || unicode.is_ascii_whitespace() {
                input_bar_key_event.grab_focus();
                let entry_text = input_bar_key_event.text();
                let mut position = entry_text.len() as i32;
                input_bar_key_event.insert_text(unicode.to_string().as_str(), &mut position);
                input_bar_key_event.set_position(position);
            }
        } else if key == gdk::Key::Up {
            if let Some(row) = list_box_key_event.selected_row() {
                if let Some(previous_row) = row.prev_sibling() {
                    previous_row.activate();
                } else if let Some(last_row) = list_box_key_event.last_child() {
                    last_row.activate();
                }
            }
        } else if key == gdk::Key::Down {
            if let Some(row) = list_box_key_event.selected_row() {
                if let Some(next_row) = row.next_sibling() {
                    next_row.activate();
                } else if let Some(first_row) = list_box_key_event.first_child() {
                    first_row.activate();
                }
            }
        }
        gtk::glib::Propagation::Stop
    });
    list_box.add_controller(key_enter_controller);

    let app_key_event = app.clone();
    let list_box_key_event = list_box.clone();
    let enter_controller = EventControllerKey::new();
    enter_controller.connect_key_pressed(move |_, key, _, _| {
        println!("key: {}", key);
        if key == gdk::Key::Escape {
            app_key_event.quit();
        } else if key == gdk::Key::Down {
            let first_child = list_box_key_event.first_child();
            if let Some(child) = first_child {
                child.activate();
                list_box_key_event.grab_focus();
            }
        } else if key == gdk::Key::Up {
            let last_child = list_box_key_event.last_child();
            if let Some(child) = last_child {
                child.activate();
                list_box_key_event.grab_focus();
            }
        }
        gtk::glib::Propagation::Stop
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
