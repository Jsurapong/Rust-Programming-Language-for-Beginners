fn main(){
    let a:(i32,bool,f64) = (200,true,8.5);

    print(a);
}

fn print(x:(i32,bool,f64)) {

    let (a,b,c) = x;


    println!("{} {} {}",a,b,c);
}