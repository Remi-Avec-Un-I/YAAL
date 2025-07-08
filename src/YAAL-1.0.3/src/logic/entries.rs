use std::ffi::{c_char, CStr};
use std::fmt;
use std::path::Path;
use serde::{Deserialize, Serialize};

use crate::loader::loader::{Entry, Plugin};

#[derive(Clone)]
#[allow(dead_code)]
pub struct IndexedEntry {
    pub entry: Entry,
    pub plugin: Plugin,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PluginConfig {
    pub name: String,
    pub prefix: String,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub yaal: YaalConfig,
    #[serde(rename = "plugins")]
    pub plugins: Option<Vec<PluginConfig>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct YaalConfig {
    pub height: u32,
    pub width: u32,
    pub maximized: bool,
    pub resizable: bool,
    pub fullscreened: bool,
    pub custom_css: Option<String>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config {{ yaal: {:?}, plugins: {:?} }}",
            self.yaal, self.plugins)
    }
}

impl Config {
    pub fn load(path: &Path) -> Self {
        let content = std::fs::read_to_string(path).unwrap_or_else(|e| {
            println!("Could not load config file: {}", e);
            std::process::exit(1);
        });
        let config = toml::from_str(&content).unwrap_or_else(|e| {
            println!("Could not parse config file: {} - Error: {}", path.display(), e);
            std::process::exit(1);
        });
        config
    }
}

pub fn query_entries(plugins: Vec<Plugin>, query: String) -> Vec<IndexedEntry> {
    let mut query_entries = Vec::new();
    let prefix = query.split(" ").next().unwrap_or("");
    
    for plugin in plugins.iter() {
        let default_prefix = unsafe { CStr::from_ptr(plugin.info.default_prefix).to_string_lossy() };
        
        if default_prefix.is_empty() || default_prefix == prefix || prefix.starts_with(default_prefix.to_string().as_str()) {
            let query_cstr = format!("{}\0", query);
            let entries = unsafe { (plugin.get_entries)(query_cstr.as_ptr() as *const c_char) };
            for i in 0..entries.length {
                let entry = unsafe { &*entries.entries.add(i) };
                query_entries.push(IndexedEntry { entry: *entry, plugin: plugin.clone() });
            }
        }
    }
    query_entries
}

pub fn get_config(path: &Path) -> Config {
    let config = Config::load(path);
    config
}