
#[cfg(windows)]
use winapi::um::{fileapi::WriteFile, errhandlingapi::GetLastError};

use std::ffi::{CString, c_void};
use crate::error::InternalError;

#[cfg(linux)]
pub(crate) unsafe fn send(pipe_handle: *mut c_void, message: String) -> Result<(), InternalError> {
    Ok(())
}

// This code is used.
#[allow(dead_code)]
#[cfg(windows)]
pub(crate) unsafe fn send(pipe_handle: *mut c_void, message: String) -> Result<(), InternalError> {

    let mut bytes_written = 0;
    let bytes_to_write = message.len() as u32;
    let message = match CString::new(message) { Ok(v) => v, Err(_) => return Err(InternalError::StringError) };

    WriteFile(
        pipe_handle,
        message.as_ptr() as *const c_void,
        bytes_to_write,
        &mut bytes_written,
        std::ptr::null_mut()
    );

    let error = GetLastError();
    match error {
        0 => (),
        2 => return Err(InternalError::InvalidHandle),
        232 => return Err(InternalError::PipeBroken),
        _ => return Err(InternalError::OsError(error))
    };

    if bytes_to_write != bytes_written { return Err(InternalError::FaultyWrite{ expected: bytes_to_write, result: bytes_written } ) } 

    Ok(())
}


