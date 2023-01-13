use std::io;

fn main() {
    let mut name = String::new();
    let mut age = String::new();
    let mut ch = String::new();

    println!("Enter your name and Age");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");

    let age: i32 = age.trim().parse().expect("Failed");
    println!("Do you want to continue? (y/n)");

    io::stdin().read_line(&mut ch).expect("Failed to read line");

    if ch.trim() == "y" {
        if age < 10 {
            println!("Your age is less than 10");
        } else {
            println!("Your Account is created");
            println!("Do you want to upload photo? (y/n)");

            io::stdin().read_line(&mut ch).expect("Failed to read line");

            if ch.trim() == "y" {
                if age < 13 {
                    println!("You cannot upload photo");
                } else {
                    println!("Your photo is uploaded");
                }
            } else {
                println!("Your photo is not uploaded");
                println!("Thanks for visiting");
            }
        }
    } else {
        println!("Oh! Let's go alone!");
    }
}
