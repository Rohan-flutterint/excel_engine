use crate::ui::app::ExcelApp;

pub struct Tabs;

impl Tabs {
    pub fn draw(_app: &mut ExcelApp, ctx: &egui::Context) {
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Sheets:");

                let sheet_count = 1; // expandable later
                for i in 0..sheet_count {
                    let label = format!("Sheet{}", i + 1);
                    ui.button(label);
                }
            });
        });
    }
}
