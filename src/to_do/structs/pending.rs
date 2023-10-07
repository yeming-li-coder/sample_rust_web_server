use super::super::enums::TaskStatus;
use super::base::Base;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let super_struct = Base {
            title: input_title.to_string(),
            status: TaskStatus::PENDING,
        };
        Pending { super_struct }
    }
}
