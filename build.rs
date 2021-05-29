use std::process::Command;

fn main() {
    
    static ARGS: [&str; 25] = [
        "--crate-name", "console_worker",
        "--edition=2018",
        r"src\bin\console-worker\main.rs",
        "--error-format=json",
        "--json=diagnostic-rendered-ansi",
        "--crate-type", "bin",
        "--emit=dep-info,link",
        "-C", "embed-bitcode=no",
        "-C", "debuginfo=2",
        "-C", "metadata=86e41bc2002d0d28",
        "--out-dir", r"C:\Users\foxcirc\3D Objects\main\projects\pipedconsole\target\debug\deps",
        "-C", r"incremental=C:\Users\foxcirc\3D Objects\main\projects\pipedconsole\target\debug\incremental",
        "-L", r"dependency=C:\Users\foxcirc\3D Objects\main\projects\pipedconsole\target\debug\deps",
        "--extern", r"pipedconsole=C:\Users\foxcirc\3D Objects\main\projects\pipedconsole\target\debug\deps\libpipedconsole-99534bac03f8bf1c.rlib",
        "--extern", r"winapi=C:\Users\foxcirc\3D Objects\main\projects\pipedconsole\target\debug\deps\libwinapi-3d48ab7a0a5ca346.rlib",
    ];

    //* build the console-worker executable
    match Command::new("rustc").args(&ARGS).output() {
        Ok(out) => {
            let out = String::from_utf8(out.stderr).unwrap_or_else(
                |err| { 
                    println!("cargo:warning=the output of rustc did contain invalid utf-8: {}", err);
                    String::from("error") //* trigger the other error message, since the check would've triggered anyways
                }
            );

            if out.len() > 0 { //* an error occured
                println!("cargo:warning=during the compilation of the console-worker executable rustc has thrown an error: {}", out)
            }
        },
        Err(err) => println!("cargo:warning=could not spawn rustc process for building the console-worker executable: {}", err)
    };

    println!("cargo:rerun-if-changed=build.rs");
}
