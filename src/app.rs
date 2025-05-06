use crate::loader::loader;
use crate::ui;
use gdk::Display;
use gtk::CssProvider;
use gtk::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn get_config_dir() -> PathBuf {
    let mut config_dir = dirs::config_dir().expect("Could not find config directory");
    config_dir.push("yaal");
    std::fs::create_dir_all(&config_dir).expect("Could not create config directory");
    config_dir
}

pub fn on_activate(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Yaal")
        .default_height(300)
        .default_width(400)
        .decorated(false)
        .resizable(false)
        .build();

    ui::window::window(&window, load_plugin(), &app);

    window.set_default_size(400, 300);
    window.present();
}

pub fn load_css() {
    let config_dir = get_config_dir();
    let css_path = config_dir.join("style.css");

    let provider = CssProvider::new();

    if css_path.exists() {
        provider.load_from_path(&css_path);
    } else {
        let mut file = File::create(&css_path).expect("Could not create CSS file");
        file.write_all(include_bytes!("default.css"))
            .expect("Could not write CSS file");
        provider.load_from_path(&css_path);
    }
    println!("Loaded CSS file : {}", css_path.display());
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn load_plugin() -> Vec<loader::Plugin> {
    let mut path = get_config_dir();
    path.push("plugins");
    std::fs::create_dir_all(&path).expect("Could not create plugins directory");
    loader::load_plugins(&path)
}
