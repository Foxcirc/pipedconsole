
use std::{
    ptr::null_mut,
    ffi::{CString, c_void}
};

use winapi::um::{
    processthreadsapi as w_ptapi,
    winbase as w_base,
    minwinbase as w_mbase,
    handleapi as w_hapi,
    fileapi as w_fapi,
    winnt as w_nt,
    errhandlingapi::GetLastError
};

use crate::{
    error::InternalError,
    ErrorKind
};

const PIPE_CONNECT_FAILED: &str = "Could not connect to the worker process's pipe.";

impl super::Console {
    /// Creates a new Console object with the specified name.
    /// 
    /// This function is currently the only way of launching a new console.
    /// It spawns a worker process wich waits for any messages
    /// from the parent and then prints them.
    /// For more information about that see [`console-worker`].
    /// 
    /// The console is closed automaticly when the returned `Console` is
    /// dropped or your program exits.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use pipedconsole::Console;
    /// # fn main() -> Result<(), pipedconsole::Error> {
    /// let my_console = Console::new("My console")?; // creates a new console window
    /// 
    /// my_console.println("Hello world!")?;
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Technical details
    /// 
    /// This method creates a worker process using the [CreateProcess] function from `winapi`
    /// and then obtains a handle to the pipe by calling the [CreateFile] function.
    /// For more information about the information in returned errors see [`crate::Error`]: pipedconsole::Error .
    /// 
    /// [CreateProcess]: https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessa
    /// [CreateFile]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
    pub fn new(name: &str) -> Result<Self, crate::Error> {

        let mut startup_info = w_ptapi::STARTUPINFOA {
            cb: 0,
            lpReserved: null_mut::<i8>(),
            lpDesktop: null_mut::<i8>(),
            lpTitle: null_mut::<i8>(),
            dwX: 0,
            dwY: 0,
            dwXSize: 0,
            dwYSize: 0,
            dwXCountChars: 0,
            dwYCountChars: 0,
            dwFillAttribute: 0,
            dwFlags: 0,
            wShowWindow: 0,
            cbReserved2: 0,
            lpReserved2: 0 as *mut u8,
            hStdInput: 0 as *mut c_void,
            hStdOutput: 0 as *mut c_void,
            hStdError: 0 as *mut c_void
        };

        let mut process_info = w_ptapi::PROCESS_INFORMATION {
            hProcess: 0 as *mut c_void,
            hThread: 0 as *mut c_void,
            dwProcessId: 0,
            dwThreadId: 0
        };

        unsafe {
            
            w_base::GetStartupInfoA(&mut startup_info);
            startup_info.lpTitle = match CString::new(name) {
                 Ok(v) => v.into_raw(),
                 Err(_) => return Err( InternalError::CStringError.into() )
            };

            startup_info.cb = std::mem::size_of_val(&startup_info) as u32;
            
            // todo test path under non-test circumstances
            let process_name = match CString::new(r"target\debug\console-worker.exe") {
                Ok(v) => v.into_raw(),
                Err(_) => return Err( InternalError::CStringError.into() )
            };

            // Create the worker process.
            w_ptapi::CreateProcessA(
                process_name,
                null_mut(),
                null_mut::<w_mbase::SECURITY_ATTRIBUTES>(), 
                null_mut::<w_mbase::SECURITY_ATTRIBUTES>(),
                0,
                w_base::NORMAL_PRIORITY_CLASS | w_base::CREATE_NEW_CONSOLE, 
                null_mut::<std::ffi::c_void>(),
                null_mut::<i8>(),
                &mut startup_info,
                &mut process_info
            );
            
            let result = GetLastError();
            match result {
                0 => (),
                2..=3 => return Err( crate::Error { message: "The path to the console-worker.exe file is invalid.".into(), kind: ErrorKind::Fatal, code: GetLastError() } ),
                _ => return Err( crate::Error { message: "The worker process could not be launched.".into(), kind: ErrorKind::Error, code: GetLastError() } )
            };

            // Retake ownership of the CStrings so they can be deallocated.
            CString::from_raw(process_name);
            CString::from_raw(startup_info.lpTitle);

            let pipe_name = match CString::new(r"\\.\pipe\pipedconsole-%PID".replace("%PID", &process_info.dwProcessId.to_string())) {
                Ok(v) => v,
                Err(_) => return Err( InternalError::CStringError.into() )
            };

            let mut pipe_handle: *mut c_void = w_hapi::INVALID_HANDLE_VALUE;
            
            // Try to connect to the named pipe wich will be opened by the other process.
            for _ in 0..8 {
                
                pipe_handle = w_fapi::CreateFileA(
                    pipe_name.as_ptr(),
                    w_nt::GENERIC_READ | w_nt::GENERIC_WRITE,
                    0,
                    null_mut(),
                    w_fapi::OPEN_EXISTING,
                    0,
                    null_mut()
                );

                let result = GetLastError();
                match result {
                    0x0 => break,
                    0x2 => std::thread::sleep(std::time::Duration::from_millis(5)),
                    _ => return Err( crate::Error { message: PIPE_CONNECT_FAILED.into(), kind: ErrorKind::Error, code: result } )
                };

            };

            if pipe_handle == w_hapi::INVALID_HANDLE_VALUE { return Err( crate::Error { message: PIPE_CONNECT_FAILED.into(), kind: ErrorKind::Error, code: 5 /* INVALID_HANDLE */ } ); };

            w_hapi::CloseHandle(process_info.hThread);
            w_hapi::CloseHandle(process_info.hProcess);

            Ok( Self {
                pid: process_info.dwProcessId,
                pipe: pipe_handle
            } )
        }
    }
}

