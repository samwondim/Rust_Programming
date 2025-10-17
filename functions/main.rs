use std::fmt;

use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut book: HashMap<&i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let complement: i32 = target - num;

        if book.contains_key(&complement) {
            return vec![*book.get(&complement).unwrap(), (i as i32)];
        } else {
            book.insert(num, i as i32);
        }
    }

    vec![]
}

fn length_of_last_word(s: String) -> i32 {
    let arr: Vec<&str> = s.split(" ").filter(|x| x.len() > 0).collect();
    arr[arr.len() - 1].len() as i32
}

struct Person<'a> {
    name: &'a str,
    age: i32,
}

impl fmt::Display for Person<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}, age: {}", self.name, self.age)
    }
}

fn main() {
    println!("hello world");
    another_function();

    let mut y = five();
    y = add_one(y);

    println!("{}", y);

    let sam = Person {
        name: "Samuel",
        age: 24,
    };

    println!(
        "The person's name is {} and they are {} yrs old.",
        sam.name, sam.age
    );
    // let test_case = vec![2, 7, 11, 15];
    //
    // let res = two_sum(test_case, 9);
    // println!("{:?}", &res);
    //
    // let output = length_of_last_word("   fly me   to   the moon  ".to_string());
    // println!("{output}");
    //
    // let s = String::from("Hello there");
    // let bytes = s.as_bytes();
    //
    // println!("{:?}", bytes);
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
