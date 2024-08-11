use std::str::FromStr;
use std::env;
fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
                        .expect("Error parsing argument"));
    }
    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}",
            numbers, d);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m !=0);
    while m!=0 {
        if m < n {
            let t: u64 = m;
            m = n;
            n = t;
        }
         m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                    3 * 7 * 11 * 13 * 19),
                3 * 11);

    assert_ne!(gcd(7,14),0);

}

/*
1. Import Necessary Modules:
    Import functionality to convert strings to numbers.
    Import functionality to handle command-line arguments.
2. Main Function:
    Create an empty list to store numbers.
3. Process Command-Line Arguments:
    For each argument provided after the program name:
        Convert the argument to a number and add it to the list.
        If conversion fails, display an error message and stop the program.
4. Check for Input:
    If no numbers are provided:
        Display a message indicating how to use the program.
        Exit the program with an error status.
5. Calculate GCD:
    Assume the first number in the list as the initial GCD.
    For each subsequent number in the list:
        Update the GCD using the current number.
6. Display Result:
    Print the list of numbers and their greatest common divisor.
7. GCD Function:
    Ensure both input numbers are non-zero.
    While the second number is not zero:
        If the second number is smaller than the first, swap them.
        Update the second number to be the remainder of the division of the first number by the second.
    Return the first number as the GCD.
*/