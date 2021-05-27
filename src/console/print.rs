use crate::{
    com::send::send,
    error::ConsoleError
};

impl super::Console {
    /// Print to the extern console.
    /// 
    /// To guarantee that the console is flushed one may call the
    /// the [`flush`]: pipedconsole::Console::flush function when done printing.
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
    pub fn print<T: ToString>(&self, message: T) -> Result<(), ConsoleError> { 
        unsafe {

            let mut message = message.to_string();
            message.push('2');

            match send(self.pipe, message) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.into())
            }
        }
    }
}
