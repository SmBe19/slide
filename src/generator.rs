use std::path::Path;
use std::error::Error;
use std::fs;
use std::borrow::Cow;

use crate::errors::{ConfigError, InvalidCommandError};

fn get_include_content(template_path: &Path, include: &str) -> std::io::Result<String> {
    let path = template_path.join("include").join(format!("{}.cpp", include));
    fs::read_to_string(path)
}

fn generate_input(config: &str) -> Result<String, ConfigError> {
    let mut res = String::new();
    Ok(res)
}

fn get_indentation(line: &str) -> String {
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

fn add_line(res: &mut String, line: &str) {
    res.push_str(line);
    res.push('\n');
}

fn add_line_indented(res: &mut String, indent: &str, line: &str) {
    add_line(res, &format!("{}{}", indent, line));
}

pub fn generate(file: &Path, template_path: &Path) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file)?;
    let mut res = String::new();
    let error = |msg| {
        Err(InvalidCommandError::new(msg).into())
    };

    let mut lines = contents.lines();

    while let Some(line) = lines.next() {
        let line_tr = line.trim();

        if line_tr.starts_with("/*!slide config") {
            add_line(&mut res, line);
            while let Some(line) = lines.next() {
                let line_tr = line.trim();
                // TODO parse config
                add_line(&mut res, line);
                if line_tr == "*/" {
                    break
                }
            }
        } else if line_tr.starts_with("//!slide") {
            let mut parts = line.split_ascii_whitespace();
            parts.next();
            match parts.next() {
                Some("include") => {
                    if let Some(include_path) = parts.next() {
                        let indent = get_indentation(line);
                        add_line_indented(&mut res, &indent, &format!("// start include from {}", include_path));
                        let included = get_include_content(template_path, include_path)?;
                        for inc_line in included.lines() {
                            add_line_indented(&mut res, &indent, inc_line);
                        }
                        add_line_indented(&mut res, &indent, &format!("// end include from {}", include_path));
                    } else {
                        return error("Missing include path");
                    }
                },
                Some("struct") => {

                },
                Some("input") => {

                },
                other => {
                    return match other {
                        Some(value) => error(value),
                        None => error("No command given"),
                    };
                }
            }
        } else {
            add_line(&mut res, line);
        }
    }

    // fs::write(file, res)?;
    println!("{}", res);

    Ok(())
}
