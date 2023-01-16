extern crate rand;
use rand::Rng;
use std::io;

fn main() {
    println!("Guess a number");

    let secret = rand::thread_rng().gen_range(1..10);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number");

        println!("You guessed: {}", guess);

        if guess < secret {
            println!("Too small");
        } else if guess > secret {
            println!("Too big");
        } else {
            println!("You win!");
            break;
        }
    }
}
