//! This is currently not used.

use pipedconsole::*;
use std::time::Instant;

const TEST_RANGE: u32 = 100_000;
const TEST_RANGE_SMALL: u32 = 500;

#[test]
fn template() {
    
    let _console = Console::new("UNNAMED").unwrap();

    let start = Instant::now();

    for _ in 0..TEST_RANGE {
        
    };
    
    let duration = Instant::now() - start;
    println!("TEMPLATE()\nTook {:?}\nThat is {:?} per call.", duration, duration / TEST_RANGE);
    read!();
}

#[test]
fn perf_flush() {
    
    let console = Console::new("PERF_FLUSH").unwrap();
    console.flush().unwrap();// warmup
    
    let start = Instant::now();
    
    for _ in 0..TEST_RANGE {
        console.flush().unwrap();
    };
    
    let duration = Instant::now() - start;
    println!("flush()\nTook {:?}\nThat is {:?} per call.", duration, duration / TEST_RANGE);
    read!();
}

// #[test]
// #[ignore]
// fn perf_is_dead() {
    
//     let console = Console::new("PERF_IS_DEAD").unwrap();
//     console.is_dead(); // warmup
    
//     let start = Instant::now();
    
//     for _ in 0..TEST_RANGE {
//         if console.is_dead() { panic!("console.is_dead() => {:?}", console.is_dead()) };
//     };
    
//     let duration = Instant::now() - start;
//     println!("is_dead()\nTook {:?}\nThat is {:?} per call.", duration, duration / TEST_RANGE);
//     read!();
// }

#[test]
fn perf_new() {
    
    let start = Instant::now();
    
    for _ in 0..TEST_RANGE_SMALL {
        Console::new("Normal").unwrap();
    };
    
    let duration = Instant::now() - start;
    println!("new()\nTook {:?}\nThat is {:?} per call.", duration, duration / TEST_RANGE_SMALL);
    read!();
}

#[macro_export]
macro_rules! read {
    () => {
        { std::io::stdin().read_line(&mut String::new()).unwrap_or_default(); }
    };
}