pub mod enums;
pub mod structs;
pub mod traits;

use enums::TaskStatus;
use structs::done::Done;
use structs::pending::Pending;

use traits::create::Create;
use traits::delete::Delete;
use traits::edit::Edit;
use traits::get::Get;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

pub fn to_do_factory(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => ItemTypes::Done(Done::new(title)),
        TaskStatus::PENDING => ItemTypes::Pending(Pending::new(title)),
    }
}

impl Get for ItemTypes {}
impl Delete for ItemTypes {}
impl Edit for ItemTypes {}
impl Create for ItemTypes {}
