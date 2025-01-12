use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::{Arc, RwLock};
use std::thread;

use libloading::{Library, Symbol};
use notify::event::{DataChange, ModifyKind};
use notify::{EventKind, RecursiveMode, Watcher};

#[derive(Debug)]

pub struct PluginLib(Option<Library>);

impl PluginLib {
    pub fn new(path: impl AsRef<str>) -> Self {
        Self {
            0: Some(unsafe { Library::new(path.as_ref()).unwrap() }),
        }
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

    pub fn close(&mut self) {
        println!("Closing plugin library");
        self.0.take().unwrap().close().unwrap();
        Self { 0: None };
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
        println!("Calling reloaded plugin function",);

        // Close current plugin
        println!("calling plugin function");
        self.plugin.write().unwrap().close();
        assert!(self.plugin.write().unwrap().0.is_none());

        // Load new plugin
        println!("{}", "#".repeat(100));

        let lib = PluginLib::new(&self.path);
        println!("Testing new plugin function");

        let plugin_function = lib.get_plugin_function().unwrap();
        let result = unsafe { plugin_function("Hello from Rust!") };

        println!("New plugin function result: {}", result);
        println!("{}", "#".repeat(100));

        // Assign new plugin
        println!("assigning new plugin");
        self.plugin.write().unwrap().0 = lib.0;
        assert!(self.plugin.write().unwrap().0.is_some());
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
                        println!("Event: {:#?}", event);
                        if let EventKind::Modify(ModifyKind::Data(DataChange::Content)) = event.kind {
                            println!("Relevant modification detected, reloading plugin... event {:?}", event);
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
    let plugin = Arc::new(Plugin::load(plugin_path));
    plugin.start_watcher();
    let mut jh_vec = vec![];
    // Main application logic

    let plugin_inner = plugin.clone();
    jh_vec.push(thread::spawn(move || {
        loop {
            let result = plugin_inner.call_plugin_function("Hello from Rust!");
            println!("Plugin function result: {:?}", result);
            let _ = thread::sleep(std::time::Duration::from_secs(5));
        }
    }));

    // let plugin_inner = plugin.clone();
    // jh_vec.push(thread::spawn(move || {
    //     let plugin = plugin_inner.clone();
    //     loop {
    //         thread::sleep(std::time::Duration::from_secs(10));
    //         println!("Manually reloading plugin... after 10 sec");
    //         plugin.reload_plugin();
    //     }
    // }));

    for jh in jh_vec {
        jh.join().unwrap();
    }
}
