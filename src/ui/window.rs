use gtk::prelude::*;

use crate::ui;
    
pub fn window(window: &gtk::ApplicationWindow) {
    let vbox = ui::widgets::vbox();
    let input = ui::widgets::input_bar();
    window.set_child(Some(&vbox));
    vbox.append(&input);
}