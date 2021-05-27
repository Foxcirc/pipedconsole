// Copyright 2021 Foxcirc.
//
// Licensed under the MIT license: https://opensource.org/licenses/MIT

//! A process wich is listening for messages, to print, read or do other stuff.
//! 
//! # What does it do?
//! 
//! This is the code for the worker process, wich is run when calling the
//! [`Console::new`] function.
//! It will create a named pipe and then wait for a client to connect
//! to it.
//! The name format for the pipe is `\\.\pipe\pipedconsole-%PID` (without escape slashes)
//! and you can launch and connect it maually if you want so.
//! 
//! After a connection is established, this process will go inside an infinite
//! loop ad listen for messages send over the pipe. If there is no
//! other process left using the pipe, the worker process will close automaticly
//! and exit with code 0.
//! 
//! For more information about named pipes, see the [microsoft docs].
//! 
//! # Syntax
//! 
//! If you want to controll this process manually you need to send it
//! specifiy commands after you've connected to the pipe.
//! 
//! You can append specific characters at the end of your message,
//! depending on what character is found the program will do different
//! things:
//! - '0' will be ignored without an error
//! - '1' will flush the `stdout` buffer
//! - '2' will print the message without a newline
//! - '3' will read from the console and send the result back trough the named pipe // todo
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
use error::InternalError;

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
                Err(InternalError::MoreData) => (),
                Err(InternalError::PipeBroken) => break,
                Err(InternalError::OsError(e)) => error("Os error {}", e),
                _ => unreachable!("receive returned something wrong")
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
