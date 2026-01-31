use crate::engine::value::Value;

#[derive(Debug, Clone)]

pub struct Cell {
    pub raw: String,
    pub value: Value,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            raw: String::new(),
            value: Value::Empty,
        }
    }
}
