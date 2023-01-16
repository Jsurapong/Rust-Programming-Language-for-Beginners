fn main() {
    let mut s = String::from("Hello");
    let b = take(s);

    println!("b is {}", b);
}

fn take(s1: String) -> String {
    println!("s1 is {}", s1);
    s1
}
