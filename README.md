# FFI Test Project

This project demonstrates how to use FFI (Foreign Function Interface) in Rust by dynamically loading and calling functions from a shared library (`libplugin01.so`). It also features automatic reloading of the library when changes are detected.

## Features

- Loads a shared library at runtime using `libloading`.
- Monitors the plugin directory for changes using `inotify`.
- Uses SHA256 to check if the library has changed (simple but effective for now).
- Automatically reloads the library and calls its function when a change is detected.
- Prints events and reload status to the console for debugging and learning purposes.

## How It Works

1. The main Rust application loads `libplugin01.so` and calls a function from it.
2. It monitors the plugin directory (not the file itself) for changes, since inotify cannot reliably watch files being written.
3. When a relevant event (like `CLOSE_WRITE`, `DELETE`, or `CREATE`) is detected, the application reloads the library and calls the function again.
4. SHA256 is used to check if the library content has actually changed before reloading.

## Example Output

```text

Plugin function result: Some("Plugin src/lib.rs:plugin01 argument passed: 00000 Hello from Rust!")
Plugin function result: Some("Plugin src/lib.rs:plugin01 argument passed: 00000 Hello from Rust!")
Plugin function result: Some("Plugin src/lib.rs:plugin01 argument passed: 00000 Hello from Rust!")
Plugin function result: Some("Plugin src/lib.rs:plugin01 argument passed: 00000 Hello from Rust!")
Plugin function result: Some("Plugin src/lib.rs:plugin01 argument passed: 00000 Hello from Rust!")
Relevant modification detected, reloading plugin... event Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(CLOSE_WRITE), cookie: 0, name: Some(".cargo-lock") }
Calling reloaded plugin function
calling plugin function
Closing plugin library
####################################################################################################
Testing new plugin function
New plugin function result: Plugin src/lib.rs:plugin01 argument passed: 11111 Hello from Rust!
####################################################################################################
assigning new plugin
Plugin function result: Some("Plugin src/lib.rs:plugin01 argument passed: 11111 Hello from Rust!")
Plugin function result: Some("Plugin src/lib.rs:plugin01 argument passed: 11111 Hello from Rust!")

```

## Notes

- Monitoring the directory is more reliable than monitoring the file itself when using inotify.
- SHA256 is used for simplicity; more robust solutions are possible.
- This project is a work in progress and intended for learning and experimentation.

## Contributing

Contributions and feedback are welcome.
