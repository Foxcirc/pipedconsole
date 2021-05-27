use winapi::um::handleapi::CloseHandle;

/// When the handle to the pipes is closed, the Console
/// application will automaticly exit.
impl Drop for super::Console {

    fn drop(&mut self) { unsafe {
        CloseHandle(self.pipe);
    } }
}
