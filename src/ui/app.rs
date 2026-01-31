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
        // TOP TOOLBAR
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    let path = std::path::Path::new("workbook.json");
                    if let Err(e) = crate::engine::storage::save_workbook(path, &self.workbook) {
                        eprintln!("Save failed: {}", e);
                    }
                }

                if ui.button("Load").clicked() {
                    let path = std::path::Path::new("workbook.json");
                    match crate::engine::storage::load_workbook(path) {
                        Ok(wb) => {
                            self.workbook = wb;
                            self.workbook.recalculate();
                        }
                        Err(e) => eprintln!("Load failed: {}", e),
                    }
                }
            });
        });

        // MAIN GRID
        Grid::draw(self, ctx);
    }
}
