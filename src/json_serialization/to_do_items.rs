use std::sync::Arc;

use crate::state::read_file;
use crate::to_do::enums::TaskStatus;
use crate::to_do::structs::base::Base;
use crate::to_do::{to_do_factory, ItemTypes};
use actix_web::http::header::ContentType;
use actix_web::{Responder, HttpResponse};
use actix_web::body::BoxBody;
use serde::Serialize;

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_items = Vec::new();
        let mut done_items = Vec::new();
        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending_items.push(packed.super_struct),
                ItemTypes::Done(packed) => done_items.push(packed.super_struct),
            };
        }
        let done_item_count = done_items.len() as i8;
        let pending_item_count = pending_items.len() as i8;
        ToDoItems {
            pending_items,
            done_items,
            done_item_count,
            pending_item_count,
        }
    }

    pub fn get_state() -> ToDoItems {
        ToDoItems::new(
            read_file("./state.json")
                .into_iter()
                .map(|(k, v)| to_do_factory(&k, TaskStatus::from_string(
                    (&v).as_str().unwrap().to_string()
                )))
                .collect(),
        )
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok().content_type(ContentType::json()).body(body)
    }
}
