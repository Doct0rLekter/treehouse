#![warn(clippy::all, clippy::pedantic)]

use std::io::{self, stdin, Write};

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

fn get_name() -> String {
    let mut your_name = String::new();
    print!("What's your name? ");
    io::stdout().flush().unwrap();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}
fn get_greeting() -> String {
    let mut greeting = String::new();
    print!("How do you want to be greeted? ");
    io::stdout().flush().unwrap();
    stdin()
        .read_line(&mut greeting)
        .expect("Failed to read line");
    println!("");
    greeting.trim().to_lowercase()
}
fn main() {
    #[derive(Debug)]
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
            println!("{}\n", self.greeting);
        }
    }

    let mut the_list = vec![
        Visitor::new("benji", "Hello, Benji. Had enough snacks for the evening?"),
        Visitor::new("ben", "One word: ice."),
        Visitor::new("jerry", "One word: cream."),
        Visitor::new("alice", "Everyone knows you are my favorite."),
    ];

    loop {
        let name = get_name();
        let known_visitor = the_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                }
                println!("{name} is not on \"the list\".\n");
                the_list.push(Visitor::new(&name, &get_greeting()));
            }
        }
    }
    println!("The final list is: {the_list:#?}");
}
