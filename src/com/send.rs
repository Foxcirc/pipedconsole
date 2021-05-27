
use winapi::um::{fileapi::WriteFile, errhandlingapi::GetLastError};
use std::ffi::{CString, c_void};
use crate::error::SendError;

/// Sends data through a pipe. (private function)
// This code is used.
#[allow(dead_code)]
pub(crate) unsafe fn send(pipe_handle: *mut c_void, message: String) -> Result<(), SendError> {

    let mut bytes_written = 0;
    let bytes_to_write = message.len() as u32;
    let message = match CString::new(message) { Ok(v) => v, Err(_) => return Err(SendError::CStringError) };

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
        2 => return Err(SendError::InvalidHandle),
        232 => return Err(SendError::PipeBroken),
        _ => return Err(SendError::OsError(error))
    };

    if bytes_to_write != bytes_written { return Err(SendError::FaultyWrite{ expected: bytes_to_write, result: bytes_written } ) } 

    Ok(())
}


