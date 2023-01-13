fn main (){
    let a = 100;
    let b = 20;
    let c = 50;

    if a>b && a>c {
        println!("a is greater");
    } else if b>c && b>a {
        println!("b is greater");
    } else {
        println!("c is greater");
    }
}