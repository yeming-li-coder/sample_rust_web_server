use serde_json::{Map, Value};

pub trait Get {
    fn get(&self, title: &str, state: &Map<String, Value>) {
        let value = state.get(title);
        match value {
            Some(result) => {
                println!("\n\nItem: {}", title);
                println!("Status: {}\n\n", result);
            }
            None => {
                println!("item: {} was not found", title);
            }
        }
    }
}
