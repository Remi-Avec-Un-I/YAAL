use gtk::prelude::*;

use crate::{loader::loader, logic::entries::bind_entries, ui};

pub fn window(window: &gtk::ApplicationWindow, plugins: Vec<loader::Plugin>) {
    let vbox = ui::widgets::vbox();
    let input = ui::widgets::input_bar();
    let indexed_entries = bind_entries(plugins);
    let list_box = ui::widgets::list_box(indexed_entries);
    window.set_child(Some(&vbox));
    vbox.append(&input);
    vbox.append(&list_box);
}
