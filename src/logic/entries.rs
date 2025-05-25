use std::ffi::{c_char, CStr};

use crate::loader::loader::{Entry, Plugin};

#[derive(Clone)]
#[allow(dead_code)]
pub struct IndexedEntry {
    pub entry: Entry,
    pub plugin: Plugin,
}

pub fn query_entries(plugins: Vec<Plugin>, query: String) -> Vec<IndexedEntry> {
    let mut query_entries = Vec::new();
    let prefix = query.split(" ").next().unwrap_or("");
    let rest = query.split(" ").skip(1).collect::<Vec<&str>>().join(" ");
    let mut rest = rest.clone();
    if rest.is_empty() {
        rest = prefix.to_string();
    }
    
    for (i, plugin) in plugins.iter().enumerate() {
        let default_prefix = unsafe { CStr::from_ptr(plugin.info.default_prefix).to_string_lossy() };
        
        if default_prefix.is_empty() || default_prefix == prefix {
            let query_cstr = format!("{}\0", rest);
            let entries = unsafe { (plugin.get_entries)(query_cstr.as_ptr() as *const c_char) };
            
            for i in 0..entries.length {
                let entry = unsafe { &*entries.entries.add(i) };
                let entry_name = unsafe { CStr::from_ptr(entry.name).to_string_lossy() };
                if entry_name.contains(&rest) {
                    query_entries.push(IndexedEntry { entry: *entry, plugin: plugin.clone() });
                }
            }
        }
    }
    query_entries
}