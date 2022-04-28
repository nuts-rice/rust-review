//declare global state using lazy_static
//requires mutex for guard on state
extern crate lazy_static;
use lazy_static::lazy_static;
use std::sync::Mutex;


lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn global_mutable() -> Result<(), String> {
    insert("apple")?;
    insert("orange")?;
    insert("peach")?;
    {
        let db = FRUIT.lock().map_err(|_| "Failed to aquire MutexGuard")?;

        db.iter().enumerate().for_each(|(i, item)| println!("{}: {}", i, item));
    }
    insert("grape")?;
    Ok(())
}

fn insert(fruit: &str) -> Result<(), String> {
    let mut db = FRUIT.lock().map_err(|_| "Failed to aquire MutexGuard")?;
    db.push(fruit.to_string());
    Ok(())
}

