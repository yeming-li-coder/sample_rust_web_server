mod to_do;

use to_do::to_do_factory;
use to_do::enums::TaskStatus;
use to_do::ItemTypes;


fn main() {
    let done = to_do_factory("washing", TaskStatus::DONE);
    
}