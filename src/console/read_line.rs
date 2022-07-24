use crate::{
    com::{send::{send, sendc, sendstr}, receive::receive},
    error::InternalError
};
use std::ffi::CStr;

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
    /// my_console.read_line(&mut buffer)?;
    /// println!("You entered {}", &buffer);
    /// 
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_line(&self, target: &mut String) -> Result<usize, crate::Error> { 
        unsafe { 

            sendc(self.pipe, 3)?;
            sendstr(self.pipe, String::new())?;

            let mut buffer = [0i8; 1024];
            let len = receive(self.pipe, buffer.as_mut_ptr(), 1024)?;

            match CStr::from_ptr(buffer.as_ptr()).to_str() {
                Ok(v) => target.push_str(&v.replace("\u{0011}", "")),
                Err(_) => return Err( InternalError::StringError.into() )
            };

            Ok(len as usize)
        }
    }
}
