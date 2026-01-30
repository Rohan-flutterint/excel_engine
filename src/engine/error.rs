#[derive(Clone, Debug)]
pub enum ExcelError {
    Div0,
    Ref,
    Value,
    Name,
    Cycle,
}

impl std::fmt::Display for ExcelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ExcelError::Div0 => "#DIV/0!",
            ExcelError::Ref => "#REF!",
            ExcelError::Value => "#VALUE!",
            ExcelError::Name => "#NAME?",
            ExcelError::Cycle => "#CYCLE!",
        };
        write!(f, "{}", s)
    }
}
