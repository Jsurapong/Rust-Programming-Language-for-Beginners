fn main(){
    let s1 = String::from("Hello");
    // let s2 = s1;// This will cause error: value borrowed here after move
    let s2 = s1.clone();
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

}