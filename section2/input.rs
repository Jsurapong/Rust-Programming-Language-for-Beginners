use std::io;
fn main(){
    let mut input = String::new();
    println!("Enter a String");

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input:i32=input.trim().parse().expect("Failed");
    println!("{} World",input);
    
}