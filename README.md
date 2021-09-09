
# Pipedconsole

A rust crate for managing multiple consoles from one **windows** application.

Normaly a program running on windows can only use one console.
> A process can be associated with only one console, so the AllocConsole 
> function fails if the calling process already has a console.

_From the [microsoft docs]._

This crate solves this problem by providing an abstraction over a worker process
wich is controlled using named pipes.

# Usage

You can use the `Console` class to create a new console, after that
you can write to it or read a line.

```rust
use pipedconsole::Console;

let console = Console::new("My Console").expect("Failed to create a new console");
console.println("What is your name?").expect("Failed to call println"); // a seperate console window

let mut name = String::new();
console.read_line(&mut name).expect("Failed to read from the console");
println!("Your name is: {}", name); // main processe's console
```

# Documentation and download

Download the crate either directly through Cargo or visit [crates.io].
More documentation can be found on [docs.rs].

# Changelog

## 0.2.3 -> 0.3.0
- Implemented `std::io::Write` for console. This means the `flush` method is now part of that trait.
- Fixed documentation.

## 0.2.0 -> 0.2.3
- New system for building the `console_worker` executable.
  It is easier to use and comes with auto detection for the executable.
  *Note: The documentation on the root page is slightly wrong in this version.*

## 0.0.0 -> 0.2.0
Got docs.rs to work correctly.

[docs.rs]: https://docs.rs/pipedconsole
[crates.io]: https://crates.io/crates/pipedconsole
[microsoft docs]: https://docs.microsoft.com/en-us/windows/console/allocconsole
