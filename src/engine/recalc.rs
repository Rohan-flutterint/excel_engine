use std::collections::{HashMap, HashSet};

use crate::engine::{
    deps::extract_deps, evaluator::eval, parser::Parser, sheet::Sheet, topo::topo_sort,
    value::Value,
};

pub struct RecalcEngine {
    pub graph: HashMap<String, HashSet<String>>,
}

impl RecalcEngine {
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
        }
    }

    pub fn rebuild_graph(&mut self, sheet: &Sheet) {
        self.graph.clear();

        for (addr, cell) in sheet.iter_cells() {
            if cell.raw.starts_with('=') {
                let mut parser = Parser::new(&cell.raw[1..]);
                let expr = parser.parse_expr();
                let mut deps = HashSet::new();
                extract_deps(&expr, &mut deps);
                self.graph.insert(addr.clone(), deps);
            }
        }
    }

    pub fn recalc(&self, sheet: &mut Sheet) {
        let order = match topo_sort(&self.graph) {
            Ok(o) => o,
            Err(_) => {
                for (_, cell) in sheet.iter_cells_mut() {
                    cell.value = Value::Error(crate::engine::error::ExcelError::Cycle);
                }
                return;
            }
        };

        for addr in order {
            let raw = sheet.get(&addr).unwrap().raw.clone();
            let value = if raw.starts_with('=') {
                let expr = crate::engine::parser::parse_safe(&raw[1..]);
                eval(expr, sheet)
            } else if let Ok(n) = raw.parse::<f64>() {
                Value::Number(n)
            } else {
                Value::Text(raw)
            };

            sheet.get_mut(&addr).unwrap().value = value;
        }
    }
}
