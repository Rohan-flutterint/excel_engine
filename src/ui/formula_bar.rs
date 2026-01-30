use crate::ui::app::ExcelApp;

pub struct FormulaBar;

impl FormulaBar {
    pub fn draw(app: &mut ExcelApp, ctx: &egui::Context) {
        egui::TopBottomPanel::top("formula_bar").show(ctx, |ui| {
            let (r, c) = app.selected;
            let addr = format!("{}{}", (b'A' + c as u8) as char, r + 1);

            ui.horizontal(|ui| {
                ui.label(&addr);

                let sheet = app.workbook.active_sheet_mut();
                let cell = sheet.get(&addr);

                let mut text = cell.map(|c| c.raw.clone()).unwrap_or_default();

                let response = ui.text_edit_singleline(&mut text);

                if response.changed() {
                    app.edit_buffer = text.clone();
                    sheet.set(&addr, &text);
                    app.workbook.recalculate();
                }
            });
        });
    }
}
