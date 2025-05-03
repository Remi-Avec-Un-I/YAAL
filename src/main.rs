use gtk::prelude::*;

mod app;
mod loader;
mod ui;

const APP_ID: &str = "com.github.yaal";

fn main() -> glib::ExitCode { 
    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_startup(|_| app::load_css());
    app.connect_activate(app::on_activate);

    app.run()
}
