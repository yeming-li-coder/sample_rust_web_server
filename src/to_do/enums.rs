use std::fmt::Display;

use serde::{Serialize, ser::SerializeStruct};

#[derive(Clone)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match self {
            Self::DONE => "Done".to_string(),
            Self::PENDING => "Pending".to_string(),
        }
    }

    pub fn from_string(input_string: String) -> Self {
        let input_string = input_string.to_uppercase();
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("{} is not supported", input_string),
        }
    }
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stringify())
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut s = serializer.serialize_struct("TaskStatus", 1)?;
            s.serialize_field("status", &self.stringify())?;
            s.end()
    }
}