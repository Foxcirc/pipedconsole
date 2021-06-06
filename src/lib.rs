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
//! # Importand
//! This crate comes with a build script wich tries to compile the `console_worker` executable.
//! This script is important because cargo does not build binaries inside library crates, so
//! it needs to be done manually.
//! 
//! If the build script has succesfully built the `console_worker` binary, it will display
//! a warning wich contains the file location of the executable and some other useful information.
//! 
//! ** Please copy the `console_worker.exe` file into the directory where the executable(s) wich is(are)
//! calling `Console::new()` are located. ** This is likely `your_dir\target\debug` or `your_dir\target
//! \release` when building in release mode. You can disable the warning by specifying the `PCAUTOCOPY`
//! environment variable.
//! - `set PCAUTOCOPY=ignore` will disable the warning. Do this after copying the file manually.
//! - `set PCAUTOCOPY=path/to/your/target/dir` will make the build script copy the `console_worker.exe` file into the specified directory.
//! 
//! If you experience any issues with the system or have any ideas how to improve it, to make it more
//! convenient to use, please create a feature request or pull request in `github`.
//! 
//! # Additional Information
//! 
//! Creating a new [`Console`] **will create a new seperate process**. That means you will
//! see a "console-worker" process in your task manager. That process is just the console
//! listening for commands to execute.
//! 
//! In order to interface to a console-worker process using another language etc. you can
//! manually launch it and then connect to the named pipe wich is created. For more
//! information about this see the `worker` documentation.
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
