use std::any::Any;
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::{Arc, RwLock};
use std::thread;

use libloading::{Library, Symbol};
use notify::event::{DataChange, ModifyKind};
use notify::{Event, EventKind, RecursiveMode, Watcher, event};

#[derive(Debug, Clone)]

pub struct PluginLib(Arc<Option<Library>>);

impl PluginLib {
    pub fn new(path: impl AsRef<str>) -> Self {
        let lib = Arc::new(Some(unsafe {
            Library::new(path.as_ref()).expect("Failed to load library")
        }));
        Self(lib.clone())
    }

    pub fn get_plugin_function(&self) -> Option<unsafe fn(&str) -> String> {
        if let Some(lib) = self.0.as_ref() {
            let plugin_function: unsafe fn(&str) -> String = unsafe {
                let func: Symbol<unsafe fn(&str) -> String> = lib.get(b"plugin_function").unwrap();
                *func
            };
            Some(plugin_function)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Plugin {
    pub path: String,
    pub plugin: Arc<RwLock<PluginLib>>,
}

impl Plugin {
    pub fn load(lib_path: impl AsRef<str>) -> Self {
        let lib = PluginLib::new(&lib_path);
        let lib_arc = Arc::new(RwLock::new(lib));
        Self {
            plugin: lib_arc.clone(),
            path: lib_path.as_ref().to_string(),
        }
    }

    pub fn call_plugin_function(&self, arg: &str) -> Option<String> {
        let plugin_function = self.plugin.read().unwrap().get_plugin_function()?;
        let result = unsafe { plugin_function(arg) };
        Some(result)
    }

    pub fn reload_plugin(&self) {
        let lib = PluginLib::new(&self.path).clone();
        unsafe {
            println!(
                "Calling reloaded plugin function {:?}",
                lib.get_plugin_function().unwrap()("Reloaded from Rust!")
            );
        }
        self.plugin.write().unwrap().0 = lib.0;
    }

    pub fn start_watcher(&self) {
        let (tx, rx) = channel();
        let mut watcher = notify::recommended_watcher(tx).unwrap();
        let self_clone = self.clone();
        thread::spawn(move || {
            watcher
                .watch(Path::new(&self_clone.path), RecursiveMode::NonRecursive)
                .unwrap();
            loop {
                match rx.recv().unwrap() {
                    Ok(event) => {
                        if let EventKind::Modify(ModifyKind::Data(DataChange::Content)) = event.kind {
                            println!("Relevant modification detected, reloading plugin...");
                            thread::sleep(std::time::Duration::from_secs(1));
                            self_clone.reload_plugin();
                        }
                    }
                    _ => {}
                }
            }
        });
    }
}
fn main() {
    let plugin_path = "plugin01/target/release/libplugin01.so";
    let plugin = Plugin::load(plugin_path);
    plugin.start_watcher();
    // Main application logic
    loop {
        if let Some(result) = plugin.call_plugin_function("Hello from Rust!") {
            println!("Plugin function result: {}", result);
        }
        thread::sleep(std::time::Duration::from_secs(5));
    }
}
