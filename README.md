# FFI Test Project

I'm trying to learn how to use the FFI in Rust. This project is a test project to help me learn how to use the FFI.
I'm loading a shared library libplugin01.so and monitoring if it change.
On change detect I'm reloading the library and calling the function in the library.

## Work In Progress

Reload on change now is working.

using sha256 to check if the library has changed.
(not the best way to check if the library has changed, but it's working for now)

testing inotify I found that you can't really monitor a file that is being written to. So I'm monitoring the directory instead.

```Rust
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(OPEN), cookie: 0, name: Some(".cargo-lock") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(OPEN | ISDIR), cookie: 0, name: Some("deps") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(ACCESS | ISDIR), cookie: 0, name: Some("deps") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(CLOSE_NOWRITE | ISDIR), cookie: 0, name: Some("deps") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(OPEN), cookie: 0, name: Some("libplugin01.so") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(CLOSE_NOWRITE), cookie: 0, name: Some("libplugin01.so") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(DELETE), cookie: 0, name: Some("libplugin01.so") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(CREATE), cookie: 0, name: Some("libplugin01.so") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(OPEN), cookie: 0, name: Some("libplugin01.d") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(ACCESS), cookie: 0, name: Some("libplugin01.d") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(CLOSE_NOWRITE), cookie: 0, name: Some("libplugin01.d") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(CLOSE_WRITE), cookie: 0, name: Some(".cargo-lock") }
Relevant modification detected, reloading plugin... event Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(CLOSE_WRITE), cookie: 0, name: Some(".cargo-lock") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(OPEN), cookie: 0, name: Some("libplugin01.so") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(ACCESS), cookie: 0, name: Some("libplugin01.so") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(CLOSE_NOWRITE), cookie: 0, name: Some("libplugin01.so") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(OPEN), cookie: 0, name: Some("libplugin01.so") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(ACCESS), cookie: 0, name: Some("libplugin01.so") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(CLOSE_NOWRITE), cookie: 0, name: Some("libplugin01.so") }
```

Now I can see the file is deleted and created again, but I can't see the file being written to.

```Rust
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(DELETE), cookie: 0, name: Some("libplugin01.so") }
Event: Event { wd: WatchDescriptor { id: 1, fd: (Weak) }, mask: EventMask(CREATE), cookie: 0, name: Some("libplugin01.so") }
```

## Current Status

The project is currently a work in progress.
Contributions and feedback are welcome.
