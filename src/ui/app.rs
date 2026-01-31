use crate::engine::storage;
use crate::engine::workbook::Workbook;
use crate::ui::grid::Grid;

use eframe::egui;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub struct ExcelApp {
    pub workbook: Workbook,
    pub selected: (usize, usize),
    pub editing_cell: Option<(usize, usize)>,
    pub edit_buffer: String,
    // FILE STATE
    pub current_file: Option<PathBuf>,
    // AUTOSAVE
    pub last_autosave: Instant,
    // DIRTY / EXIT
    pub is_dirty: bool,
    pub show_exit_confirm: bool,
}

impl ExcelApp {
    pub fn new(workbook: Workbook) -> Self {
        Self {
            workbook,
            selected: (0, 0),
            editing_cell: None,
            edit_buffer: String::new(),

            current_file: None,
            last_autosave: Instant::now(),

            is_dirty: false,
            show_exit_confirm: false,
        }
    }
}

impl eframe::App for ExcelApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // INTERCEPT WINDOW CLOSE
        if ctx.input(|i| i.viewport().close_requested()) {
            if self.is_dirty {
                self.show_exit_confirm = true;
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }

        // TOP TOOLBAR
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // SAVE
                if ui.button("Save").clicked() {
                    let path = if let Some(existing) = &self.current_file {
                        Some(existing.clone())
                    } else {
                        rfd::FileDialog::new()
                            .add_filter("Excel JSON", &["json"])
                            .set_file_name("workbook.json")
                            .save_file()
                    };

                    if let Some(path) = path {
                        if storage::save_workbook(&path, &self.workbook).is_ok() {
                            self.current_file = Some(path);
                            self.is_dirty = false;
                        }
                    }
                }

                // LOAD
                if ui.button("Load").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Excel JSON", &["json"])
                        .pick_file()
                    {
                        if let Ok(wb) = storage::load_workbook(&path) {
                            self.workbook = wb;
                            self.workbook.recalculate();
                            self.current_file = Some(path);
                            self.is_dirty = false;
                        }
                    }
                }

                if self.is_dirty {
                    ui.label("● Unsaved changes");
                }
            });
        });

        // MAIN GRID
        Grid::draw(self, ctx);
        // AUTOSAVE (EVERY 10 seconds)
        let autosave_interval = Duration::from_secs(10);

        if self.last_autosave.elapsed() >= autosave_interval {
            let save_path = if let Some(path) = &self.current_file {
                path.clone()
            } else if let Ok(exe) = std::env::current_exe() {
                exe.parent().unwrap().join("autosave.json")
            } else {
                self.last_autosave = Instant::now();
                return;
            };

            let _ = storage::save_workbook(&save_path, &self.workbook);
            println!("Autosaved to {:?}", save_path);

            self.last_autosave = Instant::now();
        }

        // EXIT CONFIRMATION MODAL
        if self.show_exit_confirm {
            egui::Window::new("Unsaved changes")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label("You have unsaved changes. Save before exiting?");
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        // SAVE & EXIT
                        if ui.button("Save").clicked() {
                            let path = self.current_file.clone().or_else(|| {
                                rfd::FileDialog::new()
                                    .add_filter("Excel JSON", &["json"])
                                    .save_file()
                            });

                            if let Some(path) = path {
                                let _ = storage::save_workbook(&path, &self.workbook);
                                self.current_file = Some(path);
                                self.is_dirty = false;
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        }

                        // DON'T SAVE
                        if ui.button("Don't Save").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }

                        // CANCEL
                        if ui.button("Cancel").clicked() {
                            self.show_exit_confirm = false;
                        }
                    });
                });
        }
    }
}

impl Default for ExcelApp {
    fn default() -> Self {
        let workbook = Workbook::new();
        ExcelApp::new(workbook)
    }
}
