use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::{Arc, RwLock};
use std::thread;

use libloading::{Library, Symbol};
use notify::{RecursiveMode, Watcher};

#[derive(Debug, Clone)]

pub struct PluginLib(Arc<Option<Library>>);

impl PluginLib {
    pub fn new(path: impl AsRef<str>) -> Self {
        let lib = Arc::new(Some(unsafe {
            Library::new(path.as_ref()).expect("Failed to load library")
        }));
        Self(lib.clone())
    }
}

#[derive(Debug, Clone)]
pub struct PluginFunction(Arc<unsafe fn(&str) -> String>);

#[derive(Debug, Clone)]
pub struct Plugin {
    pub path: String,
    pub plugin: Arc<RwLock<PluginLib>>,
    pub plugin_function: Arc<RwLock<PluginFunction>>,
}

impl Plugin {
    pub fn load(lib_path: impl AsRef<str>) -> Self {
        let lib = PluginLib::new(&lib_path).clone();
        let plugin_function: unsafe fn(&str) -> String = unsafe {
            if let Some(lib) = lib.0.as_ref() {
                let func: Symbol<unsafe fn(&str) -> String> = lib.get(b"plugin_function").unwrap();
                *func
            } else {
                panic!("Failed to load plugin function");
            }
        };

        Self {
            plugin: Arc::new(RwLock::new(lib)),
            path: lib_path.as_ref().to_string(),
            plugin_function: Arc::new(RwLock::new(PluginFunction(Arc::new(plugin_function)))),
        }
    }

    pub fn call_plugin_function(&self, arg: &str) -> Option<String> {
        let func = self.plugin_function.clone();
        Some(unsafe { (func.read().unwrap().0)(arg) })
    }

    pub fn reload_plugin(&self) {
        let lib = PluginLib::new(&self.path).clone();
        let plugin_function: unsafe fn(&str) -> String = unsafe {
            if let Some(lib) = lib.0.as_ref() {
                let func: Symbol<unsafe fn(&str) -> String> = lib.get(b"plugin_function").unwrap();
                *func
            } else {
                panic!("Failed to load plugin function");
            }
        };

        self.plugin.write().unwrap().0 = lib.0;
        self.plugin_function.write().unwrap().0 = Arc::new(plugin_function);
    }

    pub fn start_watcher(&self) {
        let (tx, rx) = channel();
        let mut watcher = notify::recommended_watcher(tx).unwrap();

        println!("Watching for changes in {:#?}", watcher);

        let self_clone = self.clone();
        thread::spawn(move || {
            watcher
                .watch(Path::new(&self_clone.path), RecursiveMode::NonRecursive)
                .unwrap();
            loop {
                match rx.recv().unwrap() {
                    Ok(event) => {
                        println!("Reloading plugin... Event {:?}", event);
                        self_clone.reload_plugin();
                    }
                    _ => {}
                }
            }
        });
    }
}
fn main() {
    let plugin_path = "./plugins/libplugin01.so";
    let plugin = Plugin::load(plugin_path);
    plugin.start_watcher();
    // Main application logic
    loop {
        println!("Plugin : {:#?}", plugin);
        if let Some(result) = plugin.call_plugin_function("Hello from Rust!") {
            println!("Plugin function result: {}", result);
        }
        thread::sleep(std::time::Duration::from_secs(5));
    }
}
