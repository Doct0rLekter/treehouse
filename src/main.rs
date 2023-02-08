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
    let the_list = ["ben", "jerry", "alice"];

    let name = get_name();
    let mut on_the_list = false;
    for i in &the_list {
        if i == &name {
            on_the_list = true;
        }
    }
    if on_the_list {
        println!("Welcome, {name}!");
    } else {
        println!("Get out!");
    }
}
