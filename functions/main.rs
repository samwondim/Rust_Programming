fn main() {
    println!("hello world");
    another_function();

    let mut y = five();
    y = add_one(y);

    println!("{}", y);
}

fn another_function() {
    println!("Another function");
}

fn five() -> u32 {
    5
}

fn add_one(x: u32) -> u32 {
    x + 1
}
