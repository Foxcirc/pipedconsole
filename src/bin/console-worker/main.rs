// Copyright 2021 Foxcirc.
//
// Licensed under he MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>

//! A process wich is listening for messages, to print, read or do other stuff.
//! 
//! # What does it do?
//! 
//! This is the code for the worker process, wich is run when calling the
//! [`Console::new`] function.
//! It will use the first command line argument wich is passed to it,
//! as the name of a (named) pipe. Then it will create a new pipe and wait for
//! a client to connect to it.
//! 
//! After a connection is established, this process will go inside an infinite
//! loop ad listen for messages send over the pipe. If there is no
//! other process left using the pipe, the worker process will close automaticly
//! and exit with error code 0.
//! 
//! For more information about named pipes, see the [microsoft docs].
//! 
//! # More information
//! 
//! One can connect multiple clients to a worker process and even launch their own
//! process to controll the console.
//! 
//! For more documentation about the syntax, used to directly controll the process,
//! see ... (i am going to write this some day, for now just append numbers in a 
//! (0, 6) range to the end of your message and see what they do :P)
//! 
//! [microsoft docs]: https://docs.microsoft.com/en-us/windows/win32/ipc/named-pipes
//! 

#![deny(missing_docs)]
#![warn(missing_doc_code_examples)]
#![deny(missing_debug_implementations)]

#[doc(hidden)]
#[path = r"..\..\com.rs"]
mod com;

#[doc(hidden)]
#[path = r"error.rs"]
mod error_functions;

#[doc(hidden)]
#[path = r"..\..\error.rs"]
// This name is used inside receive and send functions and needs to
// be consistend between "console-worker" and "lib.rs"
mod error;

use winapi::{
    um::{
        winbase as w_base,
        namedpipeapi as w_npapi,
        errhandlingapi::GetLastError
    },
    shared::ntdef::HANDLE
};
use std::{
    ptr::null_mut,
    io::Write
};
use com::{receive::*};
use error_functions::{error, warning};
use error::ReceiveError;

#[doc(hidden)]
fn main () {

    let pipe_name = std::ffi::CString::new(r"\\.\pipe\pipedconsole-%PID"
                                .replace("%PID", &std::process::id().to_string()))
                                .unwrap_or_else(|_| error("Could not create CString from pipe name.", ""));
    
    let pipe_handle: HANDLE;

    // Create the pipe used for communicating with the child process.
    
    unsafe {
        
        pipe_handle = w_base::CreateNamedPipeA(
            pipe_name.as_ptr(),
            w_base::PIPE_ACCESS_DUPLEX,
            w_base::PIPE_TYPE_MESSAGE | w_base::PIPE_READMODE_MESSAGE | w_base::PIPE_WAIT,
            w_base::PIPE_UNLIMITED_INSTANCES,
            1024,
            1024,
            4000,
            null_mut()
        );
        
        let result = GetLastError();
        if result != 0 { error("Could not create pipe. (error {})", result) };
        
        w_npapi::ConnectNamedPipe(pipe_handle, null_mut());
        
        let result = GetLastError();
        if result != 0 && result != 535 { error("Could not connect to client. (error {})", result); };
        
        // Receives the data send through the pipe.
        let message = std::ffi::CString::new([17 as u8; 1024]).unwrap().into_raw();
        let mut controll: char;
        
        loop {
            
            std::ptr::write_bytes(message, 17, 1024);

            match receive(pipe_handle, message, 1024) {
                Ok(_) => (),
                Err(ReceiveError::MoreData) => (),
                Err(ReceiveError::PipeBroken) => break,
                Err(ReceiveError::Other(e)) => error("Os error {}", e)
            };
            
            let mut command = std::ffi::CStr::from_ptr(message)
                .to_str()
                .unwrap_or_else(|_| error("The message does not contain valid utf-8.", ""))
                .replace("\u{0011}", "");
            
            if command.len() <= 0 { continue; }
            
            controll = command.pop().unwrap();
            match controll {
                '0' => continue,
                '1' => std::io::stdout().flush().unwrap_or_else(|e| warning("Could not flush stdout.", e)),
                '2' => std::io::stdout().write_all(command.as_bytes()).unwrap_or_else(|e| warning("Could not write to stdout. ({})", e)),
                _ => warning("Invalid controll character: ", controll)
            }

        }

        std::ffi::CString::from_raw(message);

    }
}
