use std::io;

fn fib(mut x: u32) -> u32 {
    let mut first: u32 = 0;
    let mut second: u32 = 1;

    while x > 0 {
        second = second + first;
        first = second;

        x = x - 1;
    }

    second
}

fn main() {
    let mut input = String::new();

    println!("Input a number: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let input = input.trim().parse().expect("Input an integer!");

    println!("{} value of the fibonacci sequece is {}", input, fib(input));
}
