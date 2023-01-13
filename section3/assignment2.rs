use std::io;

fn main() {
    let mut input = String::new();

    println!("1 + 1 = ?");

    for n in 1..4 {
        input.clear();
        io::stdin().read_line(&mut input).expect("Fail");
        let input: i32 = input.trim().parse().expect("Failed");

        if input == 1 {
            println!("You guessed Correctly");
            break;
        }

        if n != 3 {
            println!("Try Again");
        } else {
            println!("You are Failed");
        }
    }
}
