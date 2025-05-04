use std::ffi::c_char;

use crate::loader::loader::{Entry, Plugin};

pub struct IndexedEntry {
    pub entry: Entry,
    pub handle_selection: unsafe extern "C" fn(*const c_char),
}

pub fn bind_entries(plugins: Vec<Plugin>) -> Vec<IndexedEntry> {
    let mut bind_entries = Vec::new();
    for plugin in plugins {
        let entries = unsafe { (plugin.get_entries)() };
        for i in 0..entries.length {
            let entry = unsafe { &*entries.entries.add(i) };
            bind_entries.push(IndexedEntry {
                entry: *entry,
                handle_selection: plugin.handle_selection,
            });
        }
    }
    bind_entries
}
