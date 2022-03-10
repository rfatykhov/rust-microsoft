fn main() {
    let mut x: i32  = 1;
    println!("The value of x is: {}", x);
    x = 2;
    println!("The value of x is: {}", x);

    let y: bool = true;
    println!("The value of y is: {}", y);
    let y: bool = false;
    println!("The value of y is: {}", y);

    const STRING: &str = "hello";
    println!("The value of string constant is: {}", STRING);
}
