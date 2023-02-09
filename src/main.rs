#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

fn get_name() -> String {
    let mut your_name = String::new();
    println!("What's your name?");
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    struct Visitor {
        name: String,
        greeting: String,
    }

    impl Visitor {
        fn new(name: &str, greeting: &str) -> Self {
            Self {
                name: name.to_lowercase(),
                greeting: greeting.to_string(),
            }
        }
        fn greet_visitor(&self) {
            println!("{}", self.greeting);
        }
    }

    let the_list = [
        Visitor::new("benji", "Hello, Benji. Had enough snacks for the evening?"),
        Visitor::new("ben", "One word: ice."),
        Visitor::new("jerry", "One word: cream."),
        Visitor::new("alice", "Everyone knows you are my favorite."),
    ];

    let name = get_name();
    let known_visitor = the_list.iter().find(|visitor| visitor.name == name);

    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("Get the, f*ck, out of our secret base!"),
    }
}
