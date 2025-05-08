use libloading::Library;
use std::{
    ffi::CStr,
    fs,
    os::raw::c_char,
    path::{Path, PathBuf},
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PluginInfo {
    pub name: *const c_char,
    pub version: *const c_char,
    pub description: *const c_char,
    pub author: *const c_char,
    pub default_prefix: *const c_char,
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

#[repr(C)]
pub struct PathsArray {
    pub paths: *const *const c_char,
    pub length: usize,
}

#[repr(C)]
pub struct RespData {
    pub paths: *const PathsArray,
    pub light_paths: *const PathsArray,
    pub local_paths: *const PathsArray,
    pub xdg_paths: *const PathsArray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ReqData {
    pub paths: bool,
    pub light_paths: bool,
    pub local_paths: bool,
    pub xdg_paths: bool,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct Plugin {
    pub info: PluginInfo,
    pub req_data: ReqData,
    pub get_entries: unsafe extern "C" fn() -> EntryList,
    pub handle_selection: unsafe extern "C" fn(selection: *const c_char) -> bool,
    pub set_data: unsafe extern "C" fn(
        paths: *const PathsArray,
        light_paths: *const PathsArray,
        local_paths: *const PathsArray,
        xdg_paths: *const PathsArray,
    ),
}

pub fn load_plugins(path: &Path) -> Vec<Plugin> {
    // preferably the ~/.config/yaal/plugins directory
    let mut plugins = Vec::new();
    let files = fs::read_dir(path).unwrap();
    for entry in files {
        match entry {
            Ok(entry) => {
                let file_path = entry.path();
                println!("{:?}", file_path);
                match entry.path().extension() {
                    Some(ext) if ext == "so" => {
                        println!("Loading {:?}", file_path);
                        plugins.push(load_plugin(&file_path));
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

fn load_plugin(path: &PathBuf) -> Plugin {
    unsafe {
        match Library::new(path) {
            Ok(lib) => {
                match (
                    lib.get::<*const PluginInfo>(b"PLUGIN_INFO"),
                    lib.get::<*const ReqData>(b"REQ_DATA"),
                    lib.get::<unsafe extern "C" fn() -> EntryList>(b"get_entries"),
                    lib.get::<unsafe extern "C" fn(selection: *const c_char) -> bool>(
                        b"handle_selection",
                    ),
                    lib.get::<unsafe extern "C" fn(
                        paths: *const PathsArray,
                        light_paths: *const PathsArray,
                        local_paths: *const PathsArray,
                        xdg_paths: *const PathsArray,
                    )>(b"set_data"),
                ) {
                    (
                        Ok(info_ptr),
                        Ok(req_data_ptr),
                        Ok(get_entries),
                        Ok(handle_selection),
                        Ok(set_data),
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
                        let plugin = Plugin {
                            info: *info,
                            req_data: **req_data_ptr,
                            get_entries: *get_entries,
                            handle_selection: *handle_selection,
                            set_data: *set_data,
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
