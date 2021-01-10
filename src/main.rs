use std::collections::HashMap;
use std::fs;

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
                    Err(why) => println!("An error occurred: {}", why),
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
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;

        serde_json::to_writer_pretty(file, &self.map)?;

        Ok(())
    }

    fn new() -> Result<Todo, std::io::Error> {
        let file = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open("db.json")?;

        match serde_json::from_reader(file) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    fn list() -> Result<(), std::io::Error> {
        let file = fs::File::open("db.json")?;

        let todos = match serde_json::from_reader(file) {
            Ok(map) => Todo { map },
            Err(e) if e.is_eof() => Todo {
                map: HashMap::new(),
            },
            Err(e) => panic!("Something went wrong: {}", e),
        };

        if todos.map.keys().len() == 0 {
            println!("No todos found")
        } else {
            for (k, v) in todos.map {
                println!("{}\t{}", k, v)
            }
        }

        Ok(())
    }
}
