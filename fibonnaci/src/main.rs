fn main() {
    for number in 0..100 {
        let x: i64 = fib(number);
        println!("[Fibonnaci: {number}] Number: {x}");
    }
}

fn fib(number: i64) -> i64 {
    if number <= 1 {
        return number;
    }

    fib(number - 1) + fib(number - 2)
}
