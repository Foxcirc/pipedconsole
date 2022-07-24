// Copyright 2021 Foxcirc.
//
// Licensed under he MIT license: https://opensource.org/licenses/MIT

//! Create multiple consoles for a **windows** application.
//! 
//! This crate makes use of windows Named Pipes to enable a process
//! to controll multiple consoles.
//!  
//! # Quick Overview
//! 
//! Normaly a program running on windows can only use one console.
//! > A process can be associated with only one console, so the AllocConsole 
//! > function fails if the calling process already has a console.
//! 
//! _From the [microsoft docs]._
//! 
//! This crate solves this problem by providing an abstraction over a worker process
//! wich is controlled using named pipes.
//! 
//! # Examples
//! 
//! You can use the [`Console`] class to create a new console, after that
//! you can write to it or read a line.
//! It is planned, that [`Console`] will also implement the `Read` and `Write` traits.
//! 
//! ```rust
//! use pipedconsole::Console;
//! 
//! let my_console = Console::new("My Console").expect("Failed to create a new console");
//! my_console.println("What is your name?"); // seperate window
//! 
//! let mut name = String::new();
//! my_console.read_line(&mut name).expect("Could not read from the console");
//! println!("Your name is: {}", name); // main processe's console
//! ```
//! When the console object is dropped or the calling program exits, the console
//! will close automaticly, unless another process also writes to it.
//! 
//! # Important
//! This crate comes with a build script wich tries to compile the `console_worker` executable.
//! You need to have the `cargo` command usable in order for it to work.
//! This script is important because cargo does not build binaries inside library crates, so
//! it needs to be done manually.
//! 
//! If the build script runs for the first time, it will display an info message.
//! 
//! When you run your program, a call to `Console::new` will do the following:
//! 
//! 1. It trys to find the `console_worker` executable **in the same directory** as the currently running one.
//!    This will always work, if you put `console_worker.exe` into the same folder as any executable calling Console::new().
//! 2. If it cant find the executable, it tries to find it at the default path cargo will put it when you build normally.
//!    This only works inside the default cargo project structure and makes it easier to just use this crate as-is.
//! 
//! If you want to move your executable and run it in a new directory because of some reason,
//! you will have to find `console_worker.exe` on your computer.
//! For more information on where you can find it, run the build script again with the `PIPED_CONSOLE_HELP`
//! environment variable set.
//! 
//! If you use `cmd`:
//! 
//! ```
//! set PIPED_CONSOLE_HELP=TRUE
//! cargo clean
//! cargo build 
//! ``` 
//! 
//! This should display a message on where the `console_worker` executable is located.
//! 
//! # Additional Information
//! 
//! Creating a new [`Console`] **will create a new seperate process**. That means you will
//! see a "console_worker" process in your task manager. That process is just the console
//! listening for commands to execute.
//! 
//! In order to interface to a console-worker process using another language etc. you can
//! manually launch it and then connect to the named pipe wich is created. For more
//! information about this see the `worker` documentation inside the repository.
//! You can find the worker binary files at `/src/bin/console_worker/main.rs`
//! 
//! [microsoft docs]: https://docs.microsoft.com/en-us/windows/console/allocconsole

#![deny(missing_docs)]
#![warn(missing_doc_code_examples)]
#![deny(missing_debug_implementations)]

#[doc(hidden)]
pub(crate) mod com;
pub(crate) mod error;
mod console;

pub use console::Console;
pub use error::{ConsoleError as Error, ErrorKind};
