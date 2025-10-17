use std::fmt;

struct Person<'a>{
    name: &'a str,
    age: i32,
}

impl fmt::Display for Person<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}, age: {}", self.name, self.age)
    }
}

fn main() {
    let sam = Person {name: "Sam", age: 24};

    println!("This person's name is {} and they are {} yrs old.", sam.name, sam.age);
}
