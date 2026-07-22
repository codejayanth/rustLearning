use std::io::{self, Write};

#[derive(Debug)]
enum Todo {
    Add,
    Delete,
    List,
    Invalid,
}


fn main() {
    loop {
        display();
        let choice = get_input_from_user();
        match choice {
            Todo::Add => {
                println!("you Addd");
            }
            Todo::Delete => {
                println!("You delete");
            }
            Todo::List => {
                println!("You list");
            }
            Todo::Invalid => {
                println!("Wrong input");
            }
        }
    }
}

fn display() {
    println!("1. Add ");
    println!("2. Delete");
    println!("3. List");
    println!("Choose an option");
    io::stdout().flush().unwrap();
}

fn get_input_from_user() -> Todo {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Not able to read");

    let choice = input.trim();
    println!("The option is: {choice}");

    match choice {
        "1" => Todo::Add,
        "2" => Todo::Delete,
        "3" => Todo::List,
        _ => Todo::Invalid,
    }


}