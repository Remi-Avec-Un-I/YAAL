use crate::loader::loader::{Entry, Plugin};

#[derive(Clone)]
#[allow(dead_code)]
pub struct IndexedEntry {
    pub entry: Entry,
    pub plugin: Plugin,
}

pub fn bind_entries(plugins: Vec<Plugin>) -> Vec<IndexedEntry> {
    let mut bind_entries = Vec::new();
    for plugin in plugins {
        let entries = unsafe { (plugin.get_entries)() };
        for i in 0..entries.length {
            let entry = unsafe { &*entries.entries.add(i) };
            bind_entries.push(IndexedEntry {
                entry: *entry,
                plugin: plugin.clone(),
            });
        }
    }
    bind_entries
}
