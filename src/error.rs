
/// The error type returned by all functions that interact with a
/// `Console`.
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum ErrorKind {
    /// Errors that could go away after several trys and are usually not that fatal.
    Warning,
    /// Various errors wich aren't really common, but can be fatal. (eg. CString::new() failure)
    Error,
    /// The `Console` cannot be used anymore and is useless. 
    Fatal,
}

/// The main error type used by this crate.
#[derive(Debug, Clone)]
pub struct Error {
    /// A message wich can be displayed to the user.
    pub message: String,
    /// The severity of the error.
    pub kind: ErrorKind,
    /// The windows error code. `0` if the error has nothing todo with a windows-api call.
    pub code: u32
}

impl std::fmt::Display for Error { 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipedConsole-Error")
            .field("message", &self.message)
            .field("kind", &self.kind)
            .field("code", &self.code)
            .finish()
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone)]
pub(crate) enum InternalError {
    CStringError,
    PipeBroken,
    MoreData,
    InvalidHandle,
    FaultyWrite {
        expected: u32, 
        result: u32
    },
    OsError(u32)
}
