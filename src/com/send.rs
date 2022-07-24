
#[cfg(windows)]
use winapi::um::{fileapi::WriteFile, errhandlingapi::GetLastError};

use std::ffi::{CString, c_void};
use crate::error::InternalError;

#[cfg(not(windows))]
pub(crate) unsafe fn send(pipe_handle: *mut c_void, message: String) -> Result<u32, InternalError> {
    Ok(0)
}

// This code is used.
#[allow(dead_code)]
#[cfg(windows)]
pub(crate) unsafe fn send(pipe_handle: *mut c_void, message: *const i8, len: usize) -> Result<u32, InternalError> {

    let mut bytes_written = 0;

    WriteFile(
        pipe_handle,
        message.cast(),
        len as u32,
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

    if len as u32 != bytes_written { return Err(InternalError::FaultyWrite{ expected: len as u32, result: bytes_written } ) } 

    Ok(bytes_written)
}

// This code is used.
#[allow(dead_code)]
#[cfg(windows)]
pub(crate) unsafe fn sendstr(pipe_handle: *mut c_void, message: String) -> Result<u32, InternalError> {

    let len = message.len();
    let raw = match CString::new(message) { Ok(v) => v, Err(_) => return Err(InternalError::StringError) };

    send(pipe_handle, raw.as_ptr(), len)

}


// This code is used.
#[allow(dead_code)]
#[cfg(windows)]
pub(crate) unsafe fn sendc(pipe_handle: *mut c_void, code: i8) -> Result<u32, InternalError> {

    send(pipe_handle, &code, 1)

}


