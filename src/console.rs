pub(crate) mod drop;
pub(crate) mod new;
pub(crate) mod print;
pub(crate) mod println;
pub(crate) mod read_line;
pub(crate) mod read;
pub(crate) mod write;

/// Used for printing and reading from an external console.
/// 
/// This struct is the main interaction point to a worker process you can controll using
/// this struct's methods like [`Self::println`] or everything else documented here.
/// 
/// # Examples
/// 
/// ```rust
/// use pipedconsole::Console;
/// # fn main() -> Result<(), u32> {
/// let my_console = Console::new("My console")?;
/// 
/// // Prints hello world on another console window.
/// my_console.println("What is your name?");
/// 
/// let mut name = String::new();
/// my_console.read_to_string(&mut name)?
/// 
/// // Prints normally on the calling processe's console.
/// println!("Your name is: {}", name);
/// # Ok(())
/// # }
/// ```
/// 
/// # Cloning
/// 
/// You can clone this struct however you want, but note that
/// all cloned instances will controll the **same console window**.
/// 
/// # Threads
/// 
/// Currently this struct does not implement Send or Sync and there is no
/// way of connecting to an already existing console, however this will
/// be added in future versions.
/// 
/// **Do not try to send this struct across threads in any way.** The handle to
/// the pipe wich is owned by a [`Console`] can currently be inherited.
#[derive(Debug, Clone)]
pub struct Console {
    /// The process id of the worker process. You can use this to further interface with
    /// the process. You *can* shutdown the worker process using this, to ensure that
    /// it is closed "correctly" even if it get's stuck, although that souldn't happen under
    /// normal conditions.
    pub pid: u32,
    pipe: *mut std::ffi::c_void,
}
