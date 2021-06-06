use std::process::Command;
use std::path::PathBuf;

fn main() {

    //* cargo will call this build script again when building console_worker
    if std::env::var("CONSOLE_BUILD_SCRIPT_ALREADY_RUNNING").is_ok() { return; };
	std::env::set_var("CONSOLE_BUILD_SCRIPT_ALREADY_RUNNING", "");

    let out_dir = std::env::var("OUT_DIR").unwrap();

    //* build the console-worker executable
    match Command::new("cargo").args(&["build", "--bin", "console_worker", "--target-dir", &out_dir]).output() {
        Ok(out) => {
            let out = match String::from_utf8(out.stderr) {
                Ok(v) => v,
                Err(e) => {
                    println!("cargo:warning=could not compile console_worker: the output of cargo did contain invalid utf-8: {}", e);
                    String::from("error") //* trigger the other error message, since this is unusual for cargo
                }
            };
            // todo could this be triggered by something else (like a weird path)?
            if out.contains("error") { //* an error occured
                println!("cargo:warning=during the compilation of the console_worker executable cargo has thrown an error: {}", out)
            }
        },
        Err(err) => println!("cargo:warning=could not spawn cargo process for building the console-worker executable: {}", err)
    };

    println!("Succesfully built \"console_worker\". Target directory is {}", &out_dir);
    
    if let Ok(path) = std::env::var("PCAUTOCOPY") {
        
        if path != "ignore" { //* "ignore" == the warning was disabled

            let mut source = PathBuf::from(out_dir);
            let mut dest = PathBuf::from(path);

            source.push("debug");
            source.push("console_worker.exe");

            dest.push("console_worker.exe");
            
            println!(r"Attempting to copy from {:?} to {:?}", &source, &dest);
            
            match std::fs::copy(&source, &dest) {
                Ok(_) => (),
                Err(e) => println!("cargo:warning=Could not copy {:?} to {:?}. Please make sure, that \"console_worker.exe\" is in the same directory as the executable calling it! Error message: {}", source, dest, e)
            };
        }

    }
    else {
        println!("cargo:warning=Please copy \"{}\\debug\\console_worker.exe\" into the directory where your executable is located. If you want to copy automaticly please set the \"PCAUTOCOPY\" environment vriable to the destination folder For more information check out docs.rs. If you want to just disable this warning, set the \"PCAUTOCOPY\" environment variable to \"ignore\". (E.g \"set PCAUTOCOPY=path/to/your/exe/dir\" to automaticly copy or \"set PCAUTOCOPY=ignore\" to just disabe the warning) You only need to copy once at the beginning or after you changed the location of your files. Just make sure \"console_worker.exe\" is in the same directory as the executable calling it.", &out_dir);
    };
    
    println!("cargo:rerun-if-env-changed=PCAUTOCOPY");    
}
