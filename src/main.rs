use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn on_activate(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title("Yaal");
    window.set_default_size(400, 300);
    window.show_all();
    let button = Button::with_label("Click me");
    window.add(&button);

    button.connect_clicked(|_| {
        println!("Button clicked!");
    });     
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.yaal")
        .build();

    app.connect_activate(on_activate);

    app.run();
}
