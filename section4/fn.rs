fn main () {
    let sum = add(2,3);

    println!("sum = {}",sum);
}

fn add(a:i32, b:i32) -> i32 {
    return a+b;
}