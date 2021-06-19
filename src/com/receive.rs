
#[cfg(windows)]
use winapi::um::{fileapi::ReadFile, errhandlingapi::GetLastError};

use std::ffi::c_void;
use crate::error::InternalError;

#[cfg(not(windows))]
pub(crate) unsafe fn receive(pipe_handle: *mut c_void, buffer: *mut i8, bytes_to_read: u32) -> Result<u32, InternalError> {
    Ok(0)
}

// This code is used.
#[allow(dead_code)]
#[cfg(windows)]
pub(crate) unsafe fn receive(pipe_handle: *mut c_void, buffer: *mut i8, bytes_to_read: u32) -> Result<u32, InternalError> {

    let mut bytes_read = 0;

    ReadFile(
        pipe_handle,
        buffer as *mut c_void,
        bytes_to_read,
        &mut bytes_read,
        std::ptr::null_mut()
    );

    let error = GetLastError();
    match error {
        0 => (),
        0x6D => return Err(InternalError::PipeBroken), // todo this does not work
        0xEA => return Err(InternalError::MoreData),
        _ => return Err(InternalError::OsError(error))
    }

    Ok(bytes_read)

}
