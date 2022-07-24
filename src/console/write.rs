
use std::io;
use crate::com::{send::{send, sendc, sendstr}, /* receive::receive */};

impl io::Write for crate::Console {
    /// Prints the data to the console.
    /// The data must be valid utf-8, otherwise `ErrorKind::InvalidInput` will be returned.
    /// If an internal error is generated, `ErrorKind::InvalidData` may be retuend.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let string = match String::from_utf8(buf.to_vec()) {
            Ok(v) => v,
            Err(_) => return Err(io::Error::from(io::ErrorKind::InvalidInput))
        };

        let written = self.print(string)?;
        
        Ok(written)
    }


    /// Force the console to flush.
    /// 
    /// This function should be called when you are done printing
    /// something, to ensure that is actually gets displayed correctly.
    /// 
    /// Usually this function only needs to be called after a call to
    /// [`crate::Console::print`]. (If no newline character is at the end of the message.)
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use pipedconsole::Console;
    /// # fn main() -> Result<(), pipedconsole::Error> {
    /// let my_console = Console::new("My Console")?;
    /// for i 0..100 {
    ///     my_console.print(i)?;
    /// };
    /// 
    /// my_console.flush()?;
    /// # Ok(())
    /// # }
    /// ```
    fn flush(&mut self) -> io::Result<()> { 
        
        unsafe { sendc(self.pipe, 1)?; }
        unsafe { sendstr(self.pipe, String::new())? };
        Ok(())
    
    }

}

