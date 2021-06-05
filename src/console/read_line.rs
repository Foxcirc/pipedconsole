use crate::{
    com::{send::send, receive::receive},
    error::InternalError
};
use std::ffi::CString;

impl super::Console {
    /// Read a line from the console. Similar to `std::io::stdin().read_to_string()`
    /// 
    /// Reads from the console until a newline (the 0xA byte) is reached, and
    /// appends them to the provided buffer.
    /// This function will block until all the input was read.
    /// 
    /// ## Examples
    /// 
    /// ```rust
    /// use pipedconsole::Console;
    /// # fn main() -> Result<(), pipedconsole::Error> {
    /// let my_console = Console::new()?;
    /// let mut buffer = String::new();
    /// 
    /// my_console.read_to_string(&mut buffer)?;
    /// println!("You entered {}", &buffer);
    /// 
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_line(&self, target: &mut String) -> Result<usize, crate::Error> { 
        unsafe { 
            send(self.pipe, "3".into())?;
            let buffer = match CString::new([17 as u8; 1024]) {
                Ok(v) => v.into_raw(),
                Err(_) => return Err( InternalError::StringError.into() )
            };
            let bytes_read = receive(self.pipe, buffer, 1024)?;

            match CString::from_raw(buffer).to_str() {
                Ok(v) => target.push_str(&v.replace("\u{0011}", "")),
                Err(_) => return Err( InternalError::StringError.into() )
            };

            Ok(bytes_read as usize)
        }
    }
}
