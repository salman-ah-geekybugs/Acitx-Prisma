use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SampleData {
    id: i32,
    text: String,
    reminder: bool,
    timestamp: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTask {
    pub text: String,
    pub reminder: bool,
    pub timestamp: String,
}