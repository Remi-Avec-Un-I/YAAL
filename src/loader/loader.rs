use libloading::Library;
use std::{
    collections::HashMap, ffi::CStr, fs, os::raw::c_char, path::{Path, PathBuf}, process::exit
};
use crate::logic::entries::get_config;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PluginInfo {
    pub name: *const c_char,
    pub version: *const c_char,
    pub description: *const c_char,
    pub author: *const c_char,
    pub default_prefix: *const c_char,
    pub default_config: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Entry {
    pub name: *const c_char,
    pub description: *const c_char,
    pub value: *const c_char,
    pub icon: *const c_char,
    pub emoji: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EntryList {
    pub entries: *const Entry,
    pub length: usize,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct Plugin {
    pub info: PluginInfo,
    pub get_entries: unsafe extern "C" fn(query: *const c_char) -> EntryList,
    pub handle_selection: unsafe extern "C" fn(selection: *const c_char) -> bool,
}

pub fn get_config_hashmap(config_path: &Path) -> HashMap<String, HashMap<String, String>> {
    let mut configs_hashmap = HashMap::new();
    let configs = get_config(config_path);
    if let Some(plugins) = configs.plugins {
        for plugin in plugins {
            let mut pairs = HashMap::new();
            for (key, value) in plugin.extra {
                pairs.insert(key, value);
            }
            pairs.insert("prefix".to_string(), plugin.prefix.clone());
            configs_hashmap.insert(plugin.name.clone(), pairs);
        }
    }
    configs_hashmap
}

pub fn load_plugins(plugins_folder: &Path, config_path: &Path) -> Vec<Plugin> {
    // preferably the ~/.config/yaal/plugins directory
    let mut plugins = Vec::new();
    let files = fs::read_dir(plugins_folder).unwrap();
    let configs = get_config_hashmap(config_path);
    for entry in files {
        match entry {
            Ok(entry) => {
                let file_path = entry.path();
                println!("{:?}", file_path);
                match entry.path().extension() {
                    Some(ext) if ext == "so" => {
                        plugins.push(load_plugin(&file_path, &configs));
                    }
                    _ => {}
                }
            }
            Err(e) => {
                println!("{:?}", e)
            }
        }
    }
    plugins
}

fn load_plugin(path: &PathBuf, configs: &HashMap<String, HashMap<String, String>>) -> Plugin {
    unsafe {
        match Library::new(path) {
            Ok(lib) => {
                match (
                    lib.get::<*const PluginInfo>(b"PLUGIN_INFO"),
                    lib.get::<unsafe extern "C" fn(query: *const c_char) -> EntryList>(b"get_entries"),
                    lib.get::<unsafe extern "C" fn(selection: *const c_char) -> bool>(b"handle_selection"),
                    lib.get::<unsafe extern "C" fn(config: *const c_char) -> bool>(b"init_config"),
                ) {
                    (
                        Ok(info_ptr),
                        Ok(get_entries),
                        Ok(handle_selection),
                        Ok(init_config),
                    ) => {
                        let info: &PluginInfo = &**info_ptr;
                        println!("Plugin Info:");
                        println!("  Name: {}", CStr::from_ptr(info.name).to_string_lossy());
                        println!(
                            "  Version: {}",
                            CStr::from_ptr(info.version).to_string_lossy()
                        );
                        println!(
                            "  Description: {}",
                            CStr::from_ptr(info.description).to_string_lossy()
                        );
                        
                        let plugin_name = CStr::from_ptr(info.name).to_string_lossy().to_string();
                        let current_config = configs.get(&plugin_name);
                        
                        if let Some(config) = current_config {
                            let config_json = serde_json::to_string(config);
                            if let Ok(config_json) = config_json {
                                let config_cstr = format!("{}\0", config_json);
                                (init_config)(config_cstr.as_ptr() as *const c_char);
                            } else {
                                println!("Failed to convert config to JSON");
                                exit(1);
                            }
                        } else {
                            let empty_config = "{}";
                            let config_cstr = format!("{}\0", empty_config);
                            (init_config)(config_cstr.as_ptr() as *const c_char);
                        }
                        
                        let plugin = Plugin {
                            info: *info,
                            get_entries: *get_entries,
                            handle_selection: *handle_selection,
                        };
                        
                        std::mem::forget(lib);
                        plugin
                    }
                    _ => panic!("Failed to load plugin's symbols"),
                }
            }
            Err(e) => panic!("Failed to load plugin library: {}", e),
        }
    }
}
