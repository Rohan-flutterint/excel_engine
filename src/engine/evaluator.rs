use crate::engine::{
    ast::Expr, error::ExcelError, reference::parse_ref, sheet::Sheet, value::Value,
};

pub fn eval(expr: Expr, sheet: &Sheet) -> Value {
    match expr {
        Expr::Number(n) => Value::Number(n),
        Expr::Cell(name) => {
            if let Some(cell) = sheet.get(&name) {
                cell.value.clone()
            } else {
                Value::Error(ExcelError::Ref)
            }
        }
        Expr::Binary(a, op, b) => {
            let av = eval(*a, sheet).as_number();
            let bv = eval(*b, sheet).as_number();
            match (av, bv) {
                (Ok(x), Ok(y)) => match op {
                    '+' => Value::Number(x + y),
                    '-' => Value::Number(x - y),
                    '*' => Value::Number(x * y),
                    '/' => {
                        if y == 0.0 {
                            Value::Error(ExcelError::Div0)
                        } else {
                            Value::Number(x / y)
                        }
                    }
                    _ => Value::Error(ExcelError::Value),
                },
                (Err(e), _) | (_, Err(e)) => Value::Error(e),
            }
        }
        Expr::Range(a, b) => {
            let (r1, c1) = match parse_ref(&a) {
                Some(v) => v,
                None => return Value::Error(crate::engine::error::ExcelError::Ref),
            };

            let (r2, c2) = match parse_ref(&b) {
                Some(v) => v,
                None => return Value::Error(crate::engine::error::ExcelError::Ref),
            };

            let mut sum = 0.0;
            for r in r1.min(r2)..=r1.max(r2) {
                for c in c1.min(c2)..=c1.max(c2) {
                    let name = format!("{}{}", (b'A' + c as u8) as char, r + 1);
                    if let Some(cell) = sheet.get(&name) {
                        sum += cell.value.as_number().unwrap_or(0.0);
                    }
                }
            }
            Value::Number(sum)
        }
        Expr::Func(name, args) => {
            let vals: Vec<f64> = args
                .into_iter()
                .map(|e| eval(e, sheet).as_number().unwrap_or(0.0))
                .collect();

            match name.as_str() {
                "SUM" => Value::Number(vals.iter().sum()),
                "AVG" => Value::Number(vals.iter().sum::<f64>() / vals.len().max(1) as f64),
                "MIN" => Value::Number(
                    *vals
                        .iter()
                        .fold(&f64::INFINITY, |a, b| if b < a { b } else { a }),
                ),
                "MAX" => Value::Number(
                    *vals
                        .iter()
                        .fold(&f64::NEG_INFINITY, |a, b| if b > a { b } else { a }),
                ),
                _ => Value::Error(ExcelError::Name),
            }
        }
    }
}
