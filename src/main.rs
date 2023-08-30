use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => {
                println!("Welcome to the treehouse, {}", self.name);
                if self.age < 18 {
                    println!("Don't serve alcohol to {}", self.name);
                }
            }
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 18 {
                    println!("Don't serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            _ => println!("You are not welcome here!"),
        }
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
    let mut visitor_list = vec![
        Visitor::new("test", VisitorAction::Accept, 37),
        Visitor::new(
            "jonathan",
            VisitorAction::AcceptWithNote {
                note: String::from("You are the most welcome!"),
            },
            31,
        ),
        Visitor::new("Jude", VisitorAction::Refuse, 99),
        Visitor::new("Cheryl", VisitorAction::Probation, 17),
    ];

    loop {
        println!("Hello, what's your name?");
        let name = what_is_your_name();
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 12));
                }
            }
        }

        println!("{:?}", visitor_list);
    }
}
