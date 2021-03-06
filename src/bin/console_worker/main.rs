// Copyright 2021 Foxcirc.
//
// Licensed under the MIT license: https://opensource.org/licenses/MIT

//! A process wich is listening for messages, to print, read or do other stuff.
//! 
//! # What does it do?
//! 
//! This is the code for the worker process, wich is run when calling the
//! `Console::new` function.
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
//! Each instruction to the worker process consists of two messages.
//! 1. message: OPCODE
//! 2. message: null-terminated string
//! For some opcodes the 2. message can be an empty string. You must always send two messages.
//! 
//! ## Opcodes
//! 
//! - 0: ignore
//! - 1: flush stdout
//! - 2: print 2. message to stdout
//! - 3: read a line and send that back through the pipe
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
    io::Write,
    ffi::CStr,
};
use com::{receive::*, send::*};
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
        
        
        loop {
            
            // get the controll code
            let mut code: i8 = 0;

            match receive(pipe_handle, &mut code, 1) {
                Ok(_) => (),
                Err(InternalError::MoreData) => (),
                Err(InternalError::PipeBroken) => break,
                Err(InternalError::OsError(e)) => error("Os error {}", e),
                _ => unreachable!("receive returned something wrong")
            };
            
            let mut raw = [0i8; 1024];
            
            match receive(pipe_handle, raw.as_mut_ptr(), 1024) {
                Ok(len) => len,
                Err(InternalError::MoreData) => 1024,
                Err(InternalError::PipeBroken) => break,
                Err(InternalError::OsError(e)) => error("Os error {}", e),
                _ => unreachable!("receive returned something wrong")
            };
            
            
            let message = CStr::from_ptr(raw.as_ptr());

            match code {
                0 => continue,
                1 => std::io::stdout().flush().unwrap_or_else(|e| warning("Could not flush stdout.", e)),
                2 => std::io::stdout().write_all(message.to_bytes()).unwrap_or_else(|e| warning("Could not write to stdout. ({})", e)),
                3 => {
                    let mut buffer = String::new();
                    std::io::stdin().read_line(&mut buffer).unwrap_or_else(|e| { warning("Could not read line into buffer. ({})", e); 0 });
                    sendstr(pipe_handle, buffer).unwrap_or_else(|e| error(&format!("Could not send response to a \"read\" command: {:?}", e), ""));
                },
                _ => warning("Invalid controll character: {}", code)
            }

        }

    }
}
