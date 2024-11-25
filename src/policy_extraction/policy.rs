use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub label: String,
    pub context: String,
    pub body: String
}

impl fmt::Display for Policy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {},\nlabel: {},\ncontext: {},\nbody: {}", self.id, self.label, self.context, self.body)
    }
}