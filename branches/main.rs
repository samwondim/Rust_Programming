fn main() {
    let n = 3;

    if n < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let n = if n > 6 { 1 } else { 9 };

    println!("the number becomes {}", n);
}
