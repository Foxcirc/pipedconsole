use std::process::Command;

fn main() {
    //* cargo will call this build script again when building console_worker
    if std::env::var("CONSOLE_BUILD_SCRIPT_ALREADY_RUNNING").is_ok() { return; };
	std::env::set_var("CONSOLE_BUILD_SCRIPT_ALREADY_RUNNING", "");
    
    let out_dir = std::env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-env-changed=PCAUTOCOPY");    
    println!("cargo:rerun-if-changed={}\\PIPEDCONSOLE_COPY_DONE.txt", &out_dir);    

    //* check if the user already copied the file
    if std::env::var("PIPEDCONSOLE_COPY_DONE").is_ok() || std::fs::File::open(out_dir.clone() + "\\PIPEDCONSOLE_COPY_DONE.txt").is_ok() {
        return;
    };

    println!(
        "cargo:warning= <<<--------------    READ FROM HERE    -------------->>>\
                        Please copy \"{}\\debug\\console_worker.exe\" into the directory where your executable is located.\
                        The \"console_worker.exe\" file must be in the same directory as any executable calling the \"Console::new()\" function.\
                        If you are done copying please set the \"PIPEDCONSOLE_COPY_DONE\" environment variable or create this file: {}\\PIPEDCONSOLE_COPY_DONE.txt", 
        &out_dir, &out_dir
    );
    
    //* build the console-worker executable
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
            if out.contains("error") || out.contains("warning") { //* an error occured
                println!("cargo:warning=during the compilation of the console_worker executable cargo has thrown an error: {}", out);
            }
        },
        Err(err) => {
            println!("cargo:warning=could not spawn cargo process for building the console-worker executable: {}", err); 
        }    
    };
    std::process::exit(1);
}
