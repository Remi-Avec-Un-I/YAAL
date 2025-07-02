use crate::loader::loader;
use crate::logic::entries::{Config, YaalConfig};
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
    let config = load_config();
    println!("Config: {}", config);
    
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Yaal")
        .maximized(true)
        .fullscreened(config.yaal.fullscreened)
        .default_height(config.yaal.height as i32)
        .default_width(config.yaal.width as i32)
        .decorated(false)
        .resizable(config.yaal.resizable)
        .build();
    
    ui::window::window(&window, load_plugin(), &app);
    window.present();
    println!("Window activated: {:?}", std::time::SystemTime::now());
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
    let config_path = get_config_dir().join("config.toml");
    loader::load_plugins(&path, &config_path)
}

pub fn load_config() -> Config {
    let config_dir = get_config_dir();
    let config_path = config_dir.join("config.toml");
    if config_path.exists() {
        let config = Config::load(&config_path);
        config
    } else {
        let mut file = File::create(&config_path).expect("Could not create config file");
        file.write_all(include_bytes!("default.toml"))
            .expect("Could not write config file");

        Config {
            yaal: YaalConfig {
                height: 300,
                width: 400,
                maximized: false,
                resizable: false,
                fullscreened: false,
                custom_css: None,
            },
            plugins: Vec::new(),
        }
    }
}