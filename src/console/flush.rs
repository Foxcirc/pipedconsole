
use crate::com::{send::send, /* receive::receive */};

impl super::Console {
    /// Force the console to flush.
    /// 
    /// This function is especially useful when using a [`ConsoleBuilder`]
    /// with the `auto_flush` attribute set to `false` because it can
    /// increase performance if you are printing **a lot**.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use pipedconsole::ConsoleBuilder;
    /// # fn main() -> Result<(), u32> {
    /// let my_console = ConsoleBuilder::new()
    ///                     .name("My Console")
    ///                     .auto_flush(false)
    ///                     .build()?;
    /// 
    /// // This will complete almost instantly.
    /// for i 0..100000 {
    ///     my_console.print(format!("Iteration: {:?}", i));
    /// };
    /// 
    /// my_console.flush();
    /// # Ok(())
    /// # }
    /// ```
    pub fn flush(&self) { unsafe {

        // #[cfg(test)]
        send(self.pipe, "1".into()).unwrap();
        
        // // todo check if this works
        // #[cfg(not(test))]
        // send(self.pipe, "2".into()).ok();
    } }
}

