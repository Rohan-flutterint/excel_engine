use crate::engine::error::ExcelError;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Text(String),
    Bool(bool),
    Error(ExcelError),
    Empty,
}

impl Value {
    pub fn as_number(&self) -> Result<f64, ExcelError> {
        match self {
            Value::Number(n) => Ok(*n),
            Value::Text(s) => s.parse().map_err(|_| ExcelError::Value),
            Value::Empty => Ok(0.0),
            Value::Error(e) => Err(e.clone()),
            _ => Err(ExcelError::Value),
        }
    }
}
