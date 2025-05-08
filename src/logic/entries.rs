use crate::loader::loader::{Entry, PathsArray, Plugin, RespData};
use ignore::WalkBuilder;
use std::{ffi::{c_char, CString}, path::PathBuf, ptr};
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
#[allow(dead_code)]
pub struct IndexedEntry {
    pub entry: Entry,
    pub plugin: Plugin,
}

pub fn bind_entries(plugins: Vec<Plugin>) -> Vec<IndexedEntry> {
    let mut bind_entries = Vec::new();
    let mut path_storage = HashMap::new();
    
    for plugin in plugins {
        let mut resp_data = RespData {
            paths: ptr::null(),
            light_paths: ptr::null(),
            local_paths: ptr::null(),
            xdg_paths: ptr::null(),
        };
        let req_data = plugin.req_data;
        if req_data.paths {
            let paths = path_storage.entry("paths").or_insert_with(|| get_paths(vec!["/"], vec![]));
            resp_data.paths = paths;
        }
        if req_data.light_paths {
            let paths = path_storage.entry("light_paths").or_insert_with(|| get_paths(
                vec!["/"], 
                vec![
                    "proc",
                    "sys",
                    "dev",
                    "run",
                    "tmp", "var/lib/docker"]));
            resp_data.light_paths = paths;
        }
        if req_data.local_paths {
            let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/"));
            let cache_path = home.join(".cache").to_string_lossy().into_owned();
            let trash = home.join(".local/share/Trash").to_string_lossy().into_owned();
            let squashfs = home.join("squashfs-root").to_string_lossy().into_owned();
            let steam = home.join(".local/share/Steam").to_string_lossy().into_owned();
            let paths = path_storage.entry("local_paths").or_insert_with(|| get_paths(
                vec!["~/"], 
                vec![&cache_path, &trash, &squashfs, &steam]));
            resp_data.local_paths = paths;
        }
        if req_data.xdg_paths {
            let xdg_dirs = xdg::BaseDirectories::new();
            let data_dirs = xdg_dirs.get_data_dirs();
            let xdg_paths: Vec<String> = data_dirs
                .iter()
                .filter_map(|p| p.join("applications").to_str().map(|s| s.to_string()))
                .collect();
            let paths = path_storage.entry("xdg_paths").or_insert_with(|| get_paths(xdg_paths.iter().map(|s| s.as_str()).collect(), vec![]));
            resp_data.xdg_paths = paths;
        }
        unsafe { (plugin.set_data)(resp_data.paths, resp_data.light_paths, resp_data.local_paths, resp_data.xdg_paths) };
        let entries = unsafe { (plugin.get_entries)() };
        if entries.entries.is_null() {
            continue;
        }
        for i in 0..entries.length {
            let entry = unsafe { &*entries.entries.add(i) };
            if entry.name.is_null() || entry.value.is_null() {
                continue;
            }
            bind_entries.push(IndexedEntry {
                entry: *entry,
                plugin: plugin.clone(),
            });
        }
    }
    bind_entries
}

pub fn get_paths(path: Vec<&str>, filter: Vec<&str>) -> PathsArray {
    let mut paths_vec = Vec::new();
    let mut paths_ptr = Vec::new();
    
    // Convert filters to HashSet for O(1) lookups
    let filter_set: HashSet<PathBuf> = filter.into_iter()
        .map(|f| {
            if f.starts_with("~/") {
                let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/"));
                home.join(&f[2..])
            } else {
                PathBuf::from(f)
            }
        })
        .collect();

    for p in path {
        // Expand home directory if path starts with ~
        let expanded_path = if p.starts_with("~/") {
            let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/"));
            home.join(&p[2..]).to_string_lossy().into_owned()
        } else {
            p.to_string()
        };

        // Create ignore patterns from our filter set
        let mut builder = WalkBuilder::new(&expanded_path);
        builder.standard_filters(false)
               .hidden(false);

        // Add custom ignore patterns
        for filter_path in &filter_set {
            if let Some(pattern) = filter_path.to_str() {
                builder.add_custom_ignore_filename(pattern);
            }
        }

        for entry in builder.build() {
            if let Ok(entry) = entry {
                let path = entry.path();
                // Double check with HashSet for exact matches
                if !filter_set.iter().any(|f| path.starts_with(f)) {
                    if let Ok(c_str) = CString::new(path.to_str().unwrap()) {
                        paths_vec.push(c_str);
                    }
                }
            }
        }
    }
    
    // Convert CStrings to raw pointers and store them
    for c_str in paths_vec {
        paths_ptr.push(c_str.into_raw());
    }
    
    // Create the PathsArray
    let length = paths_ptr.len();
    let array = PathsArray {
        paths: paths_ptr.as_ptr() as *const *const c_char,
        length,
    };
    
    // Keep the Vec alive
    std::mem::forget(paths_ptr);
    
    array
}
