use actix_web::{web, Responder};
use serde_json::{Map, Value};

use crate::{state::read_file, to_do::{ItemTypes, to_do_factory, enums::TaskStatus}, json_serialization::to_do_items::ToDoItems};

pub async fn get() -> impl Responder {
    ToDoItems::get_state()
}
