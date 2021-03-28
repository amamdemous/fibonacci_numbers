use std::io;

fn fibo_recursive(n: i64) -> i64 {
    if n <= 1 {return n};
    return fibo_recursive(n-1) + fibo_recursive(n-2);
}

fn main() {
    let mut input = String::new();

    println!("Please input a number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i64 = input.trim().parse()
        .expect("Wrong input type!");

    println!("The {} fibonacci number is {}", input, fibo_recursive(input));
}
