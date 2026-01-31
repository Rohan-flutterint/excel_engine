use crate::engine::cell::Cell;
use std::collections::HashMap;

pub struct Sheet {
    pub name: String,
    cells: HashMap<String, Cell>,
}

impl Sheet {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            cells: std::collections::HashMap::new(),
        }
    }

    pub fn set(&mut self, addr: &str, raw: &str) {
        let cell = self.cells.entry(addr.to_string()).or_insert(Cell::new());
        cell.raw = raw.to_string();
    }

    pub fn get(&self, addr: &str) -> Option<&Cell> {
        self.cells.get(addr)
    }

    pub fn get_mut(&mut self, addr: &str) -> Option<&mut Cell> {
        self.cells.get_mut(addr)
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = (&String, &Cell)> {
        self.cells.iter()
    }

    pub fn iter_cells_mut(&mut self) -> impl Iterator<Item = (&String, &mut Cell)> {
        self.cells.iter_mut()
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cells(&self) -> &std::collections::HashMap<String, crate::engine::cell::Cell> {
        &self.cells
    }
}
