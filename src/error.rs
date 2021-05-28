
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
    /// If you get this kind of error from the [`crate::Console::new()`] function, it is likely
    /// you will not be able to create a Console in the future as well.
    Fatal,
}

/// The main error type used by this crate.
#[derive(Debug, Clone)]
pub struct ConsoleError {
    /// A message wich can be displayed to the user.
    pub message: String,
    /// The severity of the error.
    pub kind: ErrorKind,
    /// The windows error code. `0` if the error has nothing todo with a windows-api call.
    pub code: u32
}

impl std::fmt::Display for ConsoleError { 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipedConsole-Error")
            .field("message", &self.message)
            .field("kind", &self.kind)
            .field("code", &self.code)
            .finish()
    }
}

impl std::error::Error for ConsoleError {}

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

impl From<InternalError> for ConsoleError {
    fn from(v: InternalError) -> ConsoleError {
        match v {
            InternalError::CStringError => ConsoleError { message: "CString::new() failed.".into(), kind: ErrorKind::Error, code: 0 },
            InternalError::FaultyWrite { expected: e, result: r} => ConsoleError { message: format!("The data is invalid. (Expected {} bytes but got {}.)", e, r), kind: ErrorKind::Warning, code: 0 },
            InternalError::InvalidHandle => ConsoleError { message: "The (pipe) handle is invalid.".into(), kind: ErrorKind::Fatal, code: 2 },
            InternalError::PipeBroken => ConsoleError { message: "The pipe to the worker process was closed.".into(), kind: ErrorKind::Fatal, code: 232 },
            InternalError::MoreData => ConsoleError { message: "The last message could not be read completely.".into(), kind: ErrorKind::Warning, code: 234 },
            InternalError::OsError(e) => ConsoleError { message: format!("Windows error {}.", e), kind: ErrorKind::Error, code: e },
        }
    }
}
