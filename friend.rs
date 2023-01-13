use std::io;

fn main(){
    let mut ch = String::new();
    println!("Are your friends coming? (y/n)");

    io::stdin().read_line(&mut ch).expect("Failed to read line");

    if ch.trim() == "y" {
        println!("Great! Let's go!");
    }
    else{
        println!("Oh! Let's go alone!");
    }
}