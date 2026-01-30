use crate::engine::workbook::Workbook;
use crate::ui::{formula_bar::FormulaBar, grid::Grid, tabs::Tabs};

pub struct ExcelApp {
    pub workbook: Workbook,
    pub selected: (usize, usize),
    pub editing: bool,
    pub edit_buffer: String,
    pub editing_cell: Option<(usize, usize)>,
}

impl Default for ExcelApp {
    fn default() -> Self {
        Self {
            workbook: Workbook::new(),
            selected: (0, 0),
            editing: false,
            edit_buffer: String::new(),
            editing_cell: None,
        }
    }
}

impl eframe::App for ExcelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Tabs::draw(self, ctx);
        FormulaBar::draw(self, ctx);
        Grid::draw(self, ctx);
    }
}
