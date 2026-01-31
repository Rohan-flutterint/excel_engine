use crate::engine::{
    persist::{PersistSheet, PersistWorkbook},
    sheet::Sheet,
    workbook::Workbook,
};
use std::{fs, path::Path};

pub fn save_workbook(path: &Path, wb: &Workbook) -> Result<(), String> {
    let mut sheets = Vec::new();

    for sheet in wb.sheets() {
        let mut cells = std::collections::HashMap::new();

        for (addr, cell) in sheet.cells() {
            if !cell.raw.is_empty() {
                cells.insert(addr.clone(), cell.raw.clone());
            }
        }

        sheets.push(PersistSheet {
            name: sheet.name().to_string(),
            cells,
        });
    }

    let persist = PersistWorkbook {
        sheets,
        active_sheet: wb.active_sheet_index(),
    };

    let json = serde_json::to_string_pretty(&persist).map_err(|e| e.to_string())?;

    fs::write(path, json).map_err(|e| e.to_string())
}

pub fn load_workbook(path: &Path) -> Result<Workbook, String> {
    let json = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let persist: PersistWorkbook = serde_json::from_str(&json).map_err(|e| e.to_string())?;

    let mut wb = Workbook::new();
    wb.sheets_mut().clear();

    for ps in persist.sheets {
        let mut sheet = Sheet::new(&ps.name);

        for (addr, raw) in ps.cells {
            sheet.set(&addr, &raw);
        }

        wb.sheets_mut().push(sheet);
    }

    wb.set_active_sheet_index(persist.active_sheet);
    wb.recalculate();

    Ok(wb)
}
