#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Cell(String),
    Binary(Box<Expr>, char, Box<Expr>),
    Func(String, Vec<Expr>),
    Range(String, String),
}
