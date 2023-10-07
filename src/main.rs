mod processes;
mod state;
mod to_do;
use serde_json::json;
use std::env;
use to_do::enums::TaskStatus;
use to_do::to_do_factory;
use to_do::traits::delete::Delete;
use to_do::traits::edit::Edit;
use to_do::traits::get::Get;
use to_do::ItemTypes;

fn main() {
    let args: Vec<String> = env::args().collect();
    let status = &args[1];
    let title = &args[2];
    let mut state = state::read_file("./state.json");
    println!("Before operation: {:?}", state);

    let status = {
        match state.get(title) {
            Some(result) => {
                result.to_string().replace('\"', "")
            },
            None => {
                "pending".to_string()
            }
        }
    };

    let item = to_do_factory(title, TaskStatus::from_string(status.to_uppercase()));
    state.insert(title.clone(), json!(status));
    println!("After operation: {:?}", state);
    state::write_to_file("./state.json", &mut state)
}
