use std::process::Command;
use std::path::PathBuf;

fn main() {

    //* cargo will call this build script again when building console_worker
    if std::env::var("CONSOLE_BUILD_SCRIPT_ALREADY_RUNNING").is_ok() { return; };

    // todo test with release builds

	std::env::set_var("CONSOLE_BUILD_SCRIPT_ALREADY_RUNNING", "");

    //* build the console-worker executable
    match Command::new("cargo").args(&["build", "--bin", "console_worker", "--target-dir", "."]).output() {
        Ok(out) => {
            let out = match String::from_utf8(out.stderr) {
                Ok(v) => v,
                Err(e) => {
                    println!("cargo:warning=could not compile console-worker: the output of cargo did contain invalid utf-8: {}", e);
                    String::from("error") //* trigger the other error message, since this is unusual for cargo
                }
            };
            // todo could this be triggered by something else (like a path)?
            if out.contains("error") { //* an error occured
                println!("cargo:warning=during the compilation of the console-worker executable cargo has thrown an error: {}", out)
            }
        },
        Err(err) => println!("cargo:warning=could not spawn rustc process for building the console-worker executable: {}", err)
    }; 

    let mut source = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    source.push("debug");
    source.push("console_worker.exe");

    let mut dest = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    dest.pop();
    dest.pop();
    dest.pop();
    dest.push("console_worker.exe");

    println!(r"Attempting to copy from {:?} to {:?}", &source, &dest);

    match std::fs::copy(source, dest) {
        Ok(_) => (),
        Err(e) => println!("cargo:warning=Could not copy \"console_worker.exe\" to the place where your executable file will be. Please make sure, that \"console_worker.exe\" is in the same directory as the executable calling it! Original error: {}", e)
    };

    println!("cargo:rerun-if-changed=build.rs");
}
