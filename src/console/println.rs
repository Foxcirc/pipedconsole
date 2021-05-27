
use crate::error::Error as ConsoleError;

impl super::Console {
    /// Print a line to the extern console.
    /// 
    /// This method appends a newline and then calls [`print`]: Console::print with that message.
    /// 
    /// ## Examples
    /// 
    /// ```rust
    /// use pipedconsole::Console;
    /// # fn main() -> Result<(), pipedconsole::Error> {
    /// let my_console = Console::new()?;
    /// 
    /// // Prints hello world on another window, no "\n" needed.
    /// my_console.println("Hello world!")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn println<T: ToString>(&self, message: T) -> Result<(), ConsoleError> { 
        self.print(message.to_string() + "\n")
    }
}
