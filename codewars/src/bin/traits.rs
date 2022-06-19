use std::fmt::Display;

fn generic_display<T: Display> (item: T) {
    println!("{}", item)
}

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T>(T,T);

enum Option<T> {
    Some(T),
    None
}

fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    todo!()
}


use std::collections::HashMap;


#[derive(Debug)]
struct Contact {
    name: String,
    email: String,
}


fn main() {
    let a: &str = "42";
    let b: i64 = 42;

    generic_display(a);
    generic_display(b);
    let imported_contacts = vec![
        Contact {
            name: "John".to_string(),
            email: "john@smith.com".to_string(),
        },
        Contact {
            name: "steve".to_string(),
            email: "steve@jobs.com".to_string(),
        },
        Contact {
            name: "John".to_string(),
            email: "john@smith.com".to_string(),
        },
    ];
   
    let unique_contacts: HashMap<String, String> = imported_contacts
    .into_iter()
    .map(|contact| (contact.email.clone(), contact.name))
    .collect();

    println!("{:?}", unique_contacts);
    
}