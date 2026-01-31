use crate::engine::{recalc::RecalcEngine, sheet::Sheet};

pub struct Workbook {
    sheets: Vec<Sheet>,
    active: usize,
    engine: RecalcEngine,
}

impl Workbook {
    pub fn new() -> Self {
        Self {
            sheets: vec![Sheet::new("Sheet1")],
            active: 0,
            engine: RecalcEngine::new(),
        }
    }

    pub fn active_sheet_mut(&mut self) -> &mut Sheet {
        &mut self.sheets[self.active]
    }

    pub fn recalculate(&mut self) {
        let sheet = &mut self.sheets[self.active];
        self.engine.rebuild_graph(sheet);
        self.engine.recalc(sheet);
    }

    pub fn sheets(&self) -> &Vec<Sheet> {
        &self.sheets
    }

    pub fn sheets_mut(&mut self) -> &mut Vec<Sheet> {
        &mut self.sheets
    }

    pub fn active_sheet_index(&self) -> usize {
        self.active
    }

    pub fn set_active_sheet_index(&mut self, idx: usize) {
        self.active = idx;
    }
}
