use std::fmt::Display;

pub enum TaskStatus {
    DONE,
    PENDING
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match self {
            Self::DONE => "Done".to_string(),
            Self::PENDING => "Pending".to_string(),
            _ => panic!()
        }
    }
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stringify())
    }
}