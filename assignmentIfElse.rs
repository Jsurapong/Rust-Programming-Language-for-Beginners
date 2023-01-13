use std::io;

fn main() {

    let mut score = String::new();

    println!("Enter your score");

    io::stdin()
        .read_line(&mut score)
        .expect("Failed to read line");

    let score: i32 = score.trim().parse().expect("Failed");


    if score >= 90 {
        println!("A");
    } else if score >= 80 {
        println!("B");
    } else if score >= 70 {
        println!("C");
    } else if score >= 60 {
        println!("D");
    } else {
        println!("F");
    }

}
