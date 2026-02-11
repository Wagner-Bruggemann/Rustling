fn main() {
    mutations();
    constant();
    shadowing();
    tuples();
    arrays();
    print_function("A new message");
}

fn mutations() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");
}

fn constant() {
    const CONSTANT: i32 = 1;
    println!("The value of CONSTANT is: {CONSTANT}");
}

fn shadowing() {
    let shadow = "Value was not shadowed";
    println!("{shadow}");

    let shadow = "Value was shadowed";
    println!("{shadow}");
}

fn tuples() {
    let tup = (1, 'a', 23.9);
    let (x, y, z) = tup;
    println!("X = {x}; Y = {y}; z = {z};");
    let y = tup.1;
    println!("Y = {y}");
}

fn arrays() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let y = array[0];
    println!("Y = {y}");
}

fn print_function(message: &str) {
    println!("Log [info] : {}", message);
}
