use winapi::um::handleapi::CloseHandle;

/// Closes the handle to the pipe.
/// When the handle is closed, the worker
/// process will automaticly exit.
impl Drop for super::Console {

    fn drop(&mut self) { unsafe {
        CloseHandle(self.pipe);
    } }
}
