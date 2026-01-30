use crate::engine::ast::Expr;
use std::collections::HashSet;

pub fn extract_deps(expr: &Expr, out: &mut HashSet<String>) {
    match expr {
        Expr::Cell(name) => {
            out.insert(name.clone());
        }
        Expr::Binary(a, _, b) => {
            extract_deps(a, out);
            extract_deps(b, out);
        }
        Expr::Func(_, args) => {
            for a in args {
                extract_deps(a, out);
            }
        }
        Expr::Range(a, b) => {
            if let (Some((r1, c1)), Some((r2, c2))) = (
                crate::engine::reference::parse_ref(a),
                crate::engine::reference::parse_ref(b),
            ) {
                for r in r1.min(r2)..=r1.max(r2) {
                    for c in c1.min(c2)..=c1.max(c2) {
                        let name = format!("{}{}", (b'A' + c as u8) as char, r + 1);
                        out.insert(name);
                    }
                }
            }
        }

        _ => {}
    }
}
