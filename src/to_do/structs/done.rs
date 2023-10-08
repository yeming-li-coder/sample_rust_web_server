use super::super::enums::TaskStatus;
use super::base::Base;

use super::super::traits::create::Create;
use super::super::traits::delete::Delete;
use super::super::traits::edit::Edit;
use super::super::traits::get::Get;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let super_struct = Base {
            title: input_title.to_string(),
            status: TaskStatus::DONE,
        };
        Done { super_struct }
    }
}

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}
impl Create for Done {}
