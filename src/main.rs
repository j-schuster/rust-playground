use std::str::FromStr; // <- this is a trait, a collection of methods that a type can implement 
use std::env; // <- module that provides fn's for interacting with the execution environment - we use args below

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0); // <- assert is a macro, the ! means a macro invocation not a fn call. If it fails, the process exits

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn main() {
    let mut numbers = Vec::new(); // <- Vec is basically an array - meant to be grown or shrunk

    for arg in env::args().skip(1) { // <- args is a fn that gives us access to the programs command line arguments - the first return val we don't care about so we skip it
        numbers.push(u64::from_str(&arg) // <- we attempt to pull the string number from arg - u64 has the from string method - if it fails it will exit
                        .expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd Number ..."); // <- print message if no args are passed and then exit gracefully
        std::process::exit(1);
    }

    let mut d = numbers[0];

    for m in &numbers[1..] { // <- & here means we are iterating over references of our vec - we're borrowing a ref to numbers
        d = gcd(d, *m); // <- * dereferences the value it refers to
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

#[test] // <- called an attribute
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}