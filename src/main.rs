use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2);

    let mut todo = Todo::new().expect("Initialization of db failed");

    match action.as_str() {
        "add" => match item {
            Some(i) => {
                todo.insert(i);
                match todo.save() {
                    Ok(_) => println!("Todo saved"),
                    Err(why) => println!("An error occurred: {}", why),
                }
            }
            None => println!("No item specified"),
        },
        "complete" => match item {
            Some(i) => match todo.complete(&i) {
                None => println!("'{}' is not present in todo list", i),
                Some(_) => match todo.save() {
                    Ok(_) => println!("Todo completed"),
                    Err(why) => println!("An error occured: {}", why),
                },
            },
            None => println!("No item specified"),
        },
        "list" => Todo::list().expect("Failed to list todos"),
        _ => println!("Action {} not found", action),
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        fs::write("db.txt", content)
    }

    fn new() -> Result<Todo, std::io::Error> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open("db.txt")?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let mut map = HashMap::new();

        for entries in content.lines() {
            let mut values = entries.split("\t");
            let key = values.next().expect("No key");
            let val = values.next().expect("No value");
            map.insert(String::from(key), val.parse::<bool>().unwrap());
        }

        Ok(Todo { map })
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    fn list() -> Result<(), std::io::Error> {
        let content = fs::read_to_string("db.txt")?;

        for entries in content.lines() {
            let mut values = entries.split("\t");
            let key = values.next().expect("No key");
            let val = values.next().expect("No value");
            println!("{}\t{}", key, val)
        }

        Ok(())
    }
}
