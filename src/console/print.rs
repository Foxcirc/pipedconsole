use crate::{
    com::send::send,
    error::{SendError, Error as ConsoleError, ErrorKind}
};

impl super::Console {
    /// Print to the extern console.
    /// 
    /// To guarantee that the console is flushed one may call the
    /// the [`flush`]: Console::flush function when done printing.
    /// 
    /// ## Examples
    /// 
    /// ```rust
    /// use pipedconsole::Console;
    /// # fn main() -> Result<(), u32> {
    /// let my_console = Console::new()?;
    /// 
    /// // Produces "Hello world!" as an output.
    /// my_console.print("Hello ");
    /// my_console.print("world!");
    /// # Ok(())
    /// # }
    /// ```
    pub fn print<T: ToString>(&self, message: T) -> Result<(), ConsoleError> { 
        unsafe {

            let mut message = message.to_string();
            message.push('2');

            match send(self.pipe, message) {
                Ok(_) => return Ok(()),
                Err(SendError::CStringError) => return Err(ConsoleError { message: "CString::new() failed.".into(), kind: ErrorKind::Error, code: 0 }),
                Err(SendError::FaultyWrite { expected: e, result: r} ) => return Err(ConsoleError { message: format!("The data was not send correctly. (Expected {} bytes but got {}.)", e, r), kind: ErrorKind::Warning, code: 0 }),
                Err(SendError::InvalidHandle) => return Err(ConsoleError { message: "The pipe handle is invalid.".into(), kind: ErrorKind::Fatal, code: 2 }),
                Err(SendError::OsError(e)) => return Err(ConsoleError { message: format!("Windows error {}.", e), kind: ErrorKind::Error, code: e }),
                Err(SendError::PipeBroken) => return Err(ConsoleError { message: "The pipe to the worker process was closed.".into(), kind: ErrorKind::Fatal, code: 232 })
            } // todo merge error types to just use the normal "Error"
        }
    }
}
