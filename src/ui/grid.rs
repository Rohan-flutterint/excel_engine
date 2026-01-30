use crate::ui::app::ExcelApp;
use egui::{Key, Sense};

const ROWS: usize = 50;
const COLS: usize = 26;
const CELL_WIDTH: f32 = 90.0;
const ROW_HEADER_WIDTH: f32 = 40.0;

pub struct Grid;

impl Grid {
    pub fn draw(app: &mut ExcelApp, ctx: &egui::Context) {
        Self::handle_keyboard(app, ctx);

        let mut pending_commit: Option<(String, String)> = None;

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                // ===== COLUMN HEADERS =====
                ui.horizontal(|ui| {
                    ui.add_space(ROW_HEADER_WIDTH);
                    for c in 0..COLS {
                        let col = (b'A' + c as u8) as char;
                        ui.add_sized(
                            [CELL_WIDTH, 20.0],
                            egui::Label::new(
                                egui::RichText::new(col.to_string()).color(egui::Color32::BLACK),
                            ),
                        );
                    }
                });

                // GRID
                for r in 0..ROWS {
                    ui.horizontal(|ui| {
                        ui.add_sized(
                            [ROW_HEADER_WIDTH, 22.0],
                            egui::Label::new(
                                egui::RichText::new((r + 1).to_string())
                                    .color(egui::Color32::BLACK),
                            ),
                        );

                        for c in 0..COLS {
                            let addr = format!("{}{}", (b'A' + c as u8) as char, r + 1);

                            let selected = app.selected == (r, c);
                            let editing = app.editing_cell == Some((r, c));

                            let display_value = if editing {
                                app.edit_buffer.clone()
                            } else {
                                let sheet = app.workbook.active_sheet_mut();
                                match sheet.get(&addr) {
                                    Some(cell) => match &cell.value {
                                        crate::engine::value::Value::Number(n) => n.to_string(),
                                        crate::engine::value::Value::Text(s) => s.clone(),
                                        crate::engine::value::Value::Bool(b) => b.to_string(), // ✅ added
                                        crate::engine::value::Value::Error(e) => e.to_string(),
                                        crate::engine::value::Value::Empty => cell.raw.clone(),
                                    },
                                    None => "".to_string(),
                                }
                            };

                            let mut text = display_value.clone();

                            // CELL BACKGROUND
                            let bg = if selected {
                                egui::Color32::from_rgb(180, 210, 255)
                            } else {
                                egui::Color32::WHITE
                            };

                            let response = egui::Frame::none()
                                .fill(bg)
                                .stroke(egui::Stroke::new(
                                    1.0,
                                    egui::Color32::from_rgb(200, 200, 200),
                                ))
                                .show(ui, |ui| {
                                    if editing {
                                        let edit_id =
                                            egui::Id::new(format!("cell_edit_{}_{}", r, c));

                                        let resp = ui.add_sized(
                                            [CELL_WIDTH, 22.0],
                                            egui::TextEdit::singleline(&mut text)
                                                .id(edit_id)
                                                .text_color(egui::Color32::BLACK)
                                                .frame(false), // ✅ removes black box
                                        );

                                        // Force focus so cursor appears
                                        ui.memory_mut(|m| m.request_focus(edit_id));

                                        resp
                                    } else {
                                        ui.add_sized(
                                            [CELL_WIDTH, 22.0],
                                            egui::Label::new(
                                                egui::RichText::new(text.clone())
                                                    .color(egui::Color32::BLACK),
                                            )
                                            .sense(Sense::click()),
                                        )
                                    }
                                })
                                .inner;

                            // UPDATE BUFFER
                            if editing && response.changed() {
                                app.edit_buffer = text.clone();
                            }

                            // CLICK = COMMIT OLD CELL + SELECT NEW =====
                            // Single click = select or edit (Excel behavior)
                            if response.clicked() {
                                // If clicking a different cell → commit old edit
                                if let Some((er, ec)) = app.editing_cell {
                                    if (er, ec) != (r, c) {
                                        let old_addr =
                                            format!("{}{}", (b'A' + ec as u8) as char, er + 1);
                                        pending_commit = Some((old_addr, app.edit_buffer.clone()));
                                        app.editing_cell = None;
                                    }
                                }

                                // If clicking same cell again → edit mode
                                if app.selected == (r, c) {
                                    app.editing_cell = Some((r, c));
                                    let sheet = app.workbook.active_sheet_mut();
                                    app.edit_buffer =
                                        sheet.get(&addr).map(|c| c.raw.clone()).unwrap_or_default();
                                }

                                // Always update selection
                                app.selected = (r, c);
                            }

                            // Single click behavior like Excel:
                            // 1st click = select
                            // 2nd click on same cell = edit
                            if response.clicked() {
                                if app.selected == (r, c) {
                                    // already selected → enter edit mode
                                    app.editing_cell = Some((r, c));
                                    let sheet = app.workbook.active_sheet_mut();
                                    app.edit_buffer =
                                        sheet.get(&addr).map(|c| c.raw.clone()).unwrap_or_default();
                                }
                            }

                            // ENTER = COMMIT
                            if editing && ui.input(|i| i.key_pressed(Key::Enter)) {
                                pending_commit = Some((addr.clone(), app.edit_buffer.clone()));
                                app.editing_cell = None;
                            }
                        }
                    });
                }
            });
        });

        // APPLY COMMIT
        if let Some((addr, value)) = pending_commit {
            let sheet = app.workbook.active_sheet_mut();
            sheet.set(&addr, &value);
            app.workbook.recalculate();
        }
    }

    fn handle_keyboard(app: &mut ExcelApp, ctx: &egui::Context) {
        let input = ctx.input(|i| i.clone());
        let old_selected = app.selected;
        let (mut r, mut c) = app.selected;

        if input.key_pressed(Key::ArrowUp) && r > 0 {
            r -= 1;
        }
        if input.key_pressed(Key::ArrowDown) && r + 1 < ROWS {
            r += 1;
        }
        if input.key_pressed(Key::ArrowLeft) && c > 0 {
            c -= 1;
        }
        if input.key_pressed(Key::ArrowRight) && c + 1 < COLS {
            c += 1;
        }

        let new_selected = (r, c);

        if old_selected != new_selected {
            if let Some((er, ec)) = app.editing_cell {
                let addr = format!("{}{}", (b'A' + ec as u8) as char, er + 1);
                let sheet = app.workbook.active_sheet_mut();
                sheet.set(&addr, &app.edit_buffer);
                app.workbook.recalculate();
                app.editing_cell = None;
            }
        }

        app.selected = new_selected;

        if input.key_pressed(Key::F2) {
            let addr = format!("{}{}", (b'A' + c as u8) as char, r + 1);
            let sheet = app.workbook.active_sheet_mut();
            app.editing_cell = Some((r, c));
            app.edit_buffer = sheet.get(&addr).map(|c| c.raw.clone()).unwrap_or_default();
        }

        if input.key_pressed(Key::Escape) {
            app.editing_cell = None;
        }
    }
}
