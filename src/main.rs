mod engine;
mod ui;

use ui::app::ExcelApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    std::panic::set_hook(Box::new(|info| {
        eprintln!("PANIC: {}", info);
    }));

    eframe::run_native(
        "Rust Excel",
        options,
        Box::new(|_cc| Box::new(ExcelApp::default())),
    )
}
