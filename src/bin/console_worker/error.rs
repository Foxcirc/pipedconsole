
pub(crate) fn error<T: ToString>(message: &str, info: T) -> ! {

    eprintln!("\nA fatal error occured in the process: {}\n\nPress <enter> to exit.", message.replace("{}", &info.to_string()));
    std::io::stdin().read_line(&mut String::new()).unwrap_or_default();
    std::process::exit(2);

}

#[allow(dead_code)]
pub(crate) fn warning<T: ToString>(message: &str, info: T) {

    println!("\nSomething went wrong: {}", message.replace("{}", &info.to_string()));

}
