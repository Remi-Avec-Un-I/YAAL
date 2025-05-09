use gtk::prelude::*;

mod app;
mod loader;
mod logic;
mod ui;

const APP_ID: &str = "com.github.yaal";

fn main() -> glib::ExitCode {
    // print the current time very precicely
    println!("Current time: {:?}", std::time::SystemTime::now());
    let app = gtk::Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| app::load_css());
    app.connect_activate(app::on_activate);
    app.run()
}
