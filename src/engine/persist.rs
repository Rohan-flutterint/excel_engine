use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct PersistWorkbook {
    pub sheets: Vec<PersistSheet>,
    pub active_sheet: usize,
}

#[derive(Serialize, Deserialize)]
pub struct PersistSheet {
    pub name: String,
    pub cells: HashMap<String, String>,
}
