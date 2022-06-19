use std::env;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"))
    }

    match check_numbers(&numbers) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            eprintln!("Usage gcd NUMBER ...");
            std::process::exit(1)
        }
    }

    let mut d: u64 = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn check_numbers(numbers: &Vec<u64>) -> Result<(), String> {
    if numbers.is_empty() {
        return Err("given empty array.".to_string());
    }

    Ok(())
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11)
}
