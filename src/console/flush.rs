
use crate::com::{send::send, /* receive::receive */};

impl super::Console {
    /// Force the console to flush.
    /// 
    /// This function should be called when you are done printing
    /// something, to ensure that is actually gets displayed correctly.
    /// 
    /// Usually this function only needs to be called after a call to
    /// [`Console::print`]. (If no newline character is at the end of the message.)
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
    pub fn flush(&self) -> Result<(), crate::Error> { 
        
        unsafe { Ok(send(self.pipe, "1".into())?) }
    }
}

