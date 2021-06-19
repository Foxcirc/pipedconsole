
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

# Why so many versions?

There were some problems getting docs.rs to work correctly, as this is my first
crate and I had no experience with documentation etc. in rust.

Every time publishing a version I was sure to have fixed the problems that come
with docs.rs and winapi crates. It turns out it took me 5 trys to get it working
correctly...

While you _can_ use any version of this crate, the newer versions contains some
documentation improvements and spelling corrections.
Overall there aren't any bigger differences between 0.1.0 -> 0.2.0

[docs.rs]: https://docs.rs
[crates.io]: https://crates.io
[microsoft docs]: https://docs.microsoft.com/en-us/windows/console/allocconsole
