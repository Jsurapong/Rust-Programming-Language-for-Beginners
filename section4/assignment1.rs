use std::io;

fn main() {
    let mut n = String::new();

    println!("Enter a number");

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Failed");

    println!("{}! = {}", n, factorial(n));
}

fn factorial(n: u32) -> u32 {
    let mut result = 1;

    for i in 1..n + 1 {
        result *= i;
    }

    result
}
