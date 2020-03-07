use std::str::Lines;
use crate::errors::InvalidCommandError;

pub fn get_indentation(line: &str) -> String {
    let mut res = String::new();
    for c in line.chars() {
        if c.is_ascii_whitespace() {
            res.push(c);
        } else {
            break
        }
    }
    res
}

pub fn add_line(res: &mut String, line: &str) {
    res.push_str(line);
    res.push('\n');
}

pub fn add_line_indented(res: &mut String, indent: &str, line: &str) {
    add_line(res, &format!("{}{}", indent, line));
}


pub fn skip_to_count<'a>(iter: &'a mut Lines, start: Option<&str>, end: &str) -> Result<&'a str, InvalidCommandError> {
    let mut counter = 1;
    while let Some(line) = iter.next() {
        let line_tr = line.trim();
        if line_tr.starts_with("//!slide") {
            let mut parts = line_tr.split_ascii_whitespace();
            parts.next();
            if let Some(cmd) = parts.next() {
                if cmd == end {
                    counter -= 1;
                    if counter == 0 {
                        return Ok(line)
                    }
                } else if Some(cmd) == start {
                    counter += 1;
                }
            }
        }
    }
    Err(InvalidCommandError::new(&format!("Missing end for {}", end)))
}

pub fn skip_to<'a>(iter: &'a mut Lines, end: &str) -> Result<&'a str, InvalidCommandError> {
    skip_to_count(iter, None, end)
}
