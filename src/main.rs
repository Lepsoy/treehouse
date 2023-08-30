use std::io::stdin;

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

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    let visitor_list: [Visitor; 2] = [
        Visitor::new("test", "You are just a test, but welcome"),
        Visitor::new("jonathan", "You are our most prized member!"),
    ];

    println!("Hello, what's your name?");
    let name = what_is_your_name();
    let mut allow_them_in = false;

    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("{} is not on the visitor list.", name),
    }

    for visitor in &visitor_list {
        if visitor.name == name {
            allow_them_in = true;
            visitor.greet_visitor();
        }
    }

    if !allow_them_in {
        println!("You are not welcome here!");
    }
}
