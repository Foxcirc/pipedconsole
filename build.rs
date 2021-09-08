use std::process::Command;

fn main() {

    if std::env::var("DOCS_RS").is_ok() { return; } //* the script fails on docs.rs

    //* cargo will call this build script again when building console_worker
    if std::env::var("CONSOLE_BUILD_SCRIPT_ALREADY_RUNNING").is_ok() { return; };
	std::env::set_var("CONSOLE_BUILD_SCRIPT_ALREADY_RUNNING", "");
    
    println!("cargo:warning= Tip: Under normal conditions, the `console_worker` executable wich is build, can be auto-detected, this may not be the case for release builds or if you want to 'ship' your executable without the normal cargo structure.");

    let out_dir = std::env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=build.rs");

    let help = std::env::var("PIPED_CONSOLE_HELP").is_ok();

    if help { println!(
        "cargo:warning= <<<--------------    READ FROM HERE    -------------->>>\
                        Please copy '{}\\debug\\console_worker.exe' into the directory where your executable is located. \
                        The 'console_worker.exe' file must be in the same directory as any executable calling the 'Console::new()' function. \
                        You can always create an issue on github if neither of this \
                        options work for you. \
        ", 
        &out_dir
    ); return }

    //* otherwise build the console-worker executable
    match Command::new("cargo").args(&["build", "--bin", "console_worker", "--target-dir", &out_dir]).output() {
        Ok(out) => {
            let out = match String::from_utf8(out.stderr) {
                Ok(v) => v,
                Err(e) => {
                    println!("cargo:warning=could not compile console_worker: the output of cargo did contain invalid utf-8: {}", e);
                    String::new()
                }
            };
            // todo could this be triggered by something else (like a weird path)?
            if out.contains("error: ") || out.contains("warning: ") { //* an error occured
                println!("cargo:warning=during the compilation of the console_worker executable cargo has thrown an error: {}", out);
            }
        },
        Err(err) => {
            println!("cargo:warning=could not spawn cargo process for building the console-worker executable: {}", err); 
        }    
    };
}
