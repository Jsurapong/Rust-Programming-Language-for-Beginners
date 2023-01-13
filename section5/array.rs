fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    print(a);
}

fn print(x: [i32; 5]) {
    for n in 0..5 {
        println!("{}", x[n]);
    }
}
