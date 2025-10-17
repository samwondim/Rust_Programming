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

fn main() {
    let test_case = vec![2, 7, 11, 15];

    let res = two_sum(test_case, 9);
    println!("{:?}", &res);

    let output = length_of_last_word("   fly me   to   the moon  ".to_string());
    println!("{output}");

    let s = String::from("Hello there");
    let bytes = s.as_bytes();

    println!("{:?}", bytes);
}
