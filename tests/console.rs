use pipedconsole::*;

#[test]
fn console_basic() {
    
    let first = Console::new("This is a basic console.").unwrap();
    first.print("Oh my god, ");
    first.println("this message traveled through a named pipe, it should be just one line by now!");
    first.println("This should be a new line.");
    
    let second = Console::new("This is another basic console.").unwrap();
    second.print("Oh my god, ");
    second.println("this message traveled through another pipe, it should be just one line by now!");
    second.println("Woooow.");
    
    read!();
}

#[test]
#[ignore]
fn console_drop() {

    let console = Console::new("This console will be dropped.").unwrap();
    console.println(r"Good bye. \oo/");

    read!();
    drop(console);
    read!();

}

const NUM_CONSOLES: usize = 10;

#[test]
#[ignore]
fn console_spam() {

    let mut consoles: Vec<Console> = Vec::new();

    for i in 0..NUM_CONSOLES {
        consoles.push(Console::new(&format!("Console {:?}", i)).unwrap());
    }
    for (i, console) in consoles.iter().enumerate() {
        console.println(&format!("Hello from Console {:?}", i));
    }

    read!();
    
}

const TEST_PRINT_RANGE: usize = 10000;

#[test]
fn console_print() {
    
    let start = std::time::Instant::now();
    
    let first = Console::new("TEST_PRINT_1").unwrap();
    let second = Console::new("TEST_PRINT_2").unwrap();
    
    
    first.println("First normal test start.").unwrap();
    second.println("Second normal test start.").unwrap();
    
    for i in 0..TEST_PRINT_RANGE {        
        first.print(&i.to_string()).unwrap();
        second.print(&i.to_string()).unwrap();
    };
    
    for i in 0..TEST_PRINT_RANGE {
        first.print(&format!("First: {:?} ", i)).unwrap();
        second.print(&format!("Second: {:?} ", i)).unwrap();
    };
    
    first.println("First normal test end.").unwrap();
    second.println("Second normal test end.").unwrap();
    
    println!("Took: {:?}", std::time::Instant::now() - start);
    read!();
}

const TEST_PRINT_RANGE_SMALL: usize = 10;

#[test]
fn console_small() {
    
    let first = Console::new("TEST_PRINT_1").unwrap();
    let second = Console::new("TEST_PRINT_2").unwrap();

    first.println(">");

    first.println("Small test.");
    
    for i in 0..TEST_PRINT_RANGE_SMALL {        
        first.println(&i.to_string());
        second.println(&i.to_string());
    };
    
    for i in 0..TEST_PRINT_RANGE_SMALL {
        first.print(&format!("First: {:?} ", i));
        second.print(&format!("Second: {:?} ", i));
    };
    
    read!();
}

const TEST_PRINT_RANGE_SLOW: usize = 20;

#[test]
fn console_slow() {

    let first = Console::new("TEST_PRINT_SLOW_1").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(200));
    let second = Console::new("TEST_PRINT_SLOW_2").unwrap();
    
    first.println("Slow test.");

    for i in 0..TEST_PRINT_RANGE_SLOW {        
        first.println(&i.to_string());
        std::thread::sleep(std::time::Duration::from_millis(50));
        second.println(&i.to_string());
        std::thread::sleep(std::time::Duration::from_millis(20));
    };
    
    for i in 0..TEST_PRINT_RANGE_SLOW {
        first.print(&format!("First: {:?} ", i));
        std::thread::sleep(std::time::Duration::from_millis(60));
        second.print(&format!("Second: {:?} ", i));
        std::thread::sleep(std::time::Duration::from_millis(30));
    };

    read!();
}

const TEST_IMMEDIATE_DROP_RANGE: usize = 20;

#[test]
fn console_immediate_drop() {

    for i in 0..TEST_IMMEDIATE_DROP_RANGE {
        Console::new(&format!("TEST_I_DROP {:?}", i)).unwrap();
    }
    
}

const TEST_LONG_NAME_LENGTH: usize = 1024;

#[test]
fn console_long_name() {

    let mut name = String::from("TEST_LONG_NAME: ");

    for _ in 0..TEST_LONG_NAME_LENGTH {
        name.push('A');
    };
    
    let console = Console::new(&name).unwrap();
    console.println("This should have a long name!");
    
    read!();

}

const TEST_LONG_MESSAGE_LENGTH: usize = 11002;

#[test]
fn console_long_message() {
    
    let mut message = String::from("LONG MESSAGE: \n");
    for _ in 0..TEST_LONG_MESSAGE_LENGTH {
        message.push('B'); 
    };

    let console = Console::new("TEST_LONG_MESSAGE").unwrap();
  
    console.println(&message);

    read!();

}

#[test]
fn console_clone() {

    let first = Console::new("Oh, no I will be copied.").unwrap();
    first.print("Hello ");

    let second = first.clone();
    second.println("world!");

    first.println("This should work.");
    second.println("This must be the same console as before.");

    read!();

}

#[test]
fn console_other_thread() {

    std::thread::spawn(move || {
        
        let console = Console::new("TEST_OTHER_THREAD").unwrap();
        console.println("I will be dropped!");
        // console.read_line(&mut String::new()).unwrap();

    });

    read!();

}

#[test]
fn console_read() {

    let first = Console::new("TEST_READ_1").unwrap();
    let second = Console::new("TEST_READ_2").unwrap();

    first.print("Reading from first console: ");
    // let mut buff = String::new();
    // first.read_line(&mut buff).unwrap();
    // println!("Result 1: {}", buff);
    
    second.println("Reading from second console with newline:");
    // let mut buff = String::new();
    // second.read_line(&mut buff).unwrap();
    // println!("Result 2: {}", buff);
    
    println!("Type here:");
    let mut buff = String::new();
    std::io::stdin().read_line(&mut buff).unwrap();
    println!("A normal read: {}", buff);

    println!("End of test!");
    read!();
}

const TEST_PULL_RANGE: usize = 50;

#[test]
fn console_pull() {

    let console = Console::new("TEST_PULL").unwrap();

    for _ in 0..TEST_PULL_RANGE {
        console.print("Type here: ");
        // let mut buff = String::new();
        // console.read_line(&mut buff).unwrap();
        // println!("Result 1: {}", buff);
    };

    println!("End of test!");
    read!();
}

#[test]
fn console_is_dead_ok() { // todo spawn thread so it is does not block when the console's reply get's stuck OR add overlapped I/O

    let console = Console::new("TEST_IS_DEAD_OK").unwrap();
    console.println("Do not close this one.");

    read!();

    // if console.is_dead() { panic!("console.is_dead => {:?}", console.is_dead()) }

}

#[test]
fn console_is_dead_err() {

    let console = Console::new("TEST_IS_DEAD_ERR").unwrap();
    console.println("Close this one.");

    read!();

    // if !console.is_dead() { panic!("console.is_dead => {:?}", console.is_dead()) }

}

#[test]
fn console_flex() {

    let console = Console::new("TEST_IS_DEAD_ERR").unwrap();
    console.println("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    console.println("empty");

    read!();

}

#[macro_export]
macro_rules! read {
    () => {
        { std::io::stdin().read_line(&mut String::new()).unwrap_or_default(); }
    };
}
