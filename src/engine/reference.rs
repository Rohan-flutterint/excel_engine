use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^([A-Z]+)([0-9]+)$").unwrap();
}

pub fn parse_ref(s: &str) -> Option<(usize, usize)> {
    let caps = RE.captures(s)?;

    let col_str = caps.get(1)?.as_str();
    let row_str = caps.get(2)?.as_str();

    let row: usize = row_str.parse::<usize>().ok()?.checked_sub(1)?;

    let mut col = 0usize;
    for c in col_str.chars() {
        col = col * 26 + (c as usize - 'A' as usize + 1);
    }

    Some((row, col - 1))
}
