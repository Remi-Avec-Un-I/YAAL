use gtk::prelude::*;

use crate::{loader::loader, ui};

pub fn window(window: &gtk::ApplicationWindow, plugins: Vec<loader::Plugin>, app: &gtk::Application) {
    let vbox = ui::widgets::vbox();
    let input = ui::widgets::input_bar();
    let list_box = ui::widgets::list_box(plugins, String::new(), &input, &app);
    window.set_child(Some(&vbox));
    vbox.append(&input);
    vbox.append(&list_box);
}
