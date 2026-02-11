const CONSTANT: i32 = 1;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    println!("The value of CONSTANT is: {CONSTANT}");

    let shadow = "Value was not shadowed";
    println!("{shadow}");

    let shadow = "Value was shadowed";
    println!("{shadow}");
}
