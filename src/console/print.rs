use crate::com::send::{send, sendc, sendstr};

impl super::Console {
    /// Print to the extern console.
    /// 
    /// To guarantee that the console is flushed one may call the
    /// the [`crate::Console::flush`] function when done printing.
    /// 
    /// ## Examples
    /// 
    /// ```rust
    /// use pipedconsole::Console;
    /// # fn main() -> Result<(), pipedconsole::Error> {
    /// let my_console = Console::new()?;
    /// 
    /// // Produces "Hello world!" as an output.
    /// my_console.print("Hello ")?;
    /// my_console.print("world!")?;
    /// my_console.flush()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn print<T: ToString>(&self, message: T) -> Result<usize, crate::Error> { 

        unsafe { sendc(self.pipe, 2)? };
        let written = unsafe { sendstr(self.pipe, message.to_string())? };
        Ok(written as usize)
    }
}
