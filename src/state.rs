use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use serde_json::{Map, Value};

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    if let Ok(json) = serde_json::from_reader::<_, Value>(reader) {
        json.as_object().unwrap().clone()
    } else {
        Map::<String, Value>::new()
    }
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let file = File::create(file_name).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, state).expect("Unable to write file");
}
