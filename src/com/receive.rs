
use std::ffi::c_void;
use winapi::um::{fileapi::ReadFile, errhandlingapi::GetLastError};
use crate::error::ReceiveError;

/// Provides an abstraction over ReadFile. (private function)
// This code is used.
#[allow(dead_code)]
pub(crate) unsafe fn receive(pipe_handle: *mut c_void, buffer: *mut i8, bytes_to_read: u32) -> Result<u32, ReceiveError> {

    // let buffer = vec![17; bytes_to_read as usize];

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
        0x6D => return Err(ReceiveError::PipeBroken),
        0xEA => return Err(ReceiveError::MoreData), // More data is available to read. This is handled later.
        _ => return Err(ReceiveError::Other(error))
    }

    Ok(bytes_read)

}
