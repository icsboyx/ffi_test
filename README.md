# FFI Test Project

I'm trying to learn how to use the FFI in Rust. This project is a test project to help me learn how to use the FFI.
I'm loading a shared library libplugin01.so and monitoring if it change.
On change detect I'm reloading the library and calling the function in the library.

## Work In Progress

Reload on change now is working.

- I'm using `notify` to monitor the file for changes.
  -- it is not working as expected. i only get, access, metadata modified, evens on lib rebuild.
  ```shell
  Event: Event {
  kind: Modify(
  Metadata(
  Any,
  ),
  ),
  paths: [
  "/data/Rust/playground/ffi_test/plugin01/target/release/libplugin01.so",
  ],
  attr:tracker: None,
  attr:flag: None,
  attr:info: None,
  attr:source: None,
  }
  Event: Event {
  kind: Access(
  Open(
  Any,
  ),
  ),
  paths: [
  "/data/Rust/playground/ffi_test/plugin01/target/release/libplugin01.so",
  ],
  attr:tracker: None,
  attr:flag: None,
  attr:info: None,
  attr:source: None,
  }
  Event: Event {
  kind: Modify(
  Metadata(
  Any,
  ),
  ),
  paths: [
  "/data/Rust/playground/ffi_test/plugin01/target/release/libplugin01.so",
  ],
  attr:tracker: None,
  attr:flag: None,
  attr:info: None,
  attr:source: None,
  }
  ```

````
    I expect to get the following event when I rebuild the library.

  ```Rust
if let EventKind::Modify(ModifyKind::Data(DataChange::Content)) = event.kind {
                            println!("Relevant modification detected, reloading plugin... event {:?}", event);
                            self_clone.reload_plugin();
                        }
````

## Current Status

The project is currently a work in progress.
Contributions and feedback are welcome.

```

```
