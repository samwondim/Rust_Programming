fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 3;

    println!("The value of x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len();

    println!("spaces {}", spaces);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is {}", y);

    let a = [1, 2, 3, 4, 5];
}
