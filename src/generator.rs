use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::str::Lines;

use crate::codegen::{read_variable, struct_definition};
use crate::errors::{ConfigError, InvalidCommandError};
use crate::plugin::{generate_plugin_code, handle_plugin_options, parse_plugin_options};
use crate::ty::{get_type, InpStruct};
use crate::util::{add_line, add_line_indented, get_indentation, skip_to};

fn get_include_content(template_path: &Path, include: &str) -> Result<String, Box<dyn Error>> {
    let path = template_path
        .join("include")
        .join(format!("{}.cpp", include));
    transform(&fs::read_to_string(path)?, template_path)
}

fn handle_plugin(
    line_tr: &str,
    template_path: &Path,
    plugins: &mut String,
    input: &mut String,
) -> Result<(), Box<dyn Error>> {
    let mut parts = line_tr.split_ascii_whitespace();

    let include_path = &parts.next().ok_or(ConfigError)?[1..];
    let included = get_include_content(template_path, include_path)?;

    let mut options = HashMap::new();
    options.insert("input", "true");

    let mut option_keys = Vec::new();

    let mut lines = included.lines();
    while let Some(line) = lines.next() {
        let line_tr = line.trim();
        if line_tr.starts_with("/*!slide plugin_config") {
            handle_plugin_options(&mut options, &mut option_keys, &mut lines)?;
            break;
        }
    }

    parse_plugin_options(&mut options, &option_keys, &mut parts)?;

    generate_plugin_code(&mut options, &included, input, plugins)?;

    Ok(())
}

fn handle_variable(
    line_tr: &str,
    input: &mut String,
    defined_structs: &HashMap<char, InpStruct>,
) -> Result<(), ConfigError> {
    for inp_config in line_tr.split_ascii_whitespace() {
        let mut parts = inp_config.split(':');
        let variable = parts.next().ok_or(ConfigError)?;
        let typ_part = parts.next().unwrap_or("i");
        let mut chars = typ_part.chars();
        let typ = get_type(&mut chars, &defined_structs).ok_or(ConfigError)?;
        read_variable(
            typ,
            variable,
            input,
            &String::from(""),
            &mut parts,
            &mut String::from(""),
        );
    }
    Ok(())
}

fn generate_code_from_config(
    template_path: &Path,
    config: &str,
    structs: &mut String,
    plugins: &mut String,
    input: &mut String,
) -> Result<(), Box<dyn Error>> {
    let mut defined_structs = HashMap::new();
    for line in config.lines() {
        let line_tr = line.trim();
        if line_tr.starts_with('}') {
            struct_definition(line_tr, structs, &mut defined_structs)?;
        } else if line_tr.starts_with('+') {
            handle_plugin(line_tr, template_path, plugins, input)?;
        } else {
            handle_variable(line_tr, input, &defined_structs)?;
        }
    }
    Ok(())
}

fn handle_config(
    template_path: &Path,
    lines: &mut Lines,
    res: &mut String,
    structs: &mut String,
    plugins: &mut String,
    input: &mut String,
) -> Result<(), Box<dyn Error>> {
    let mut config = String::new();
    while let Some(line) = lines.next() {
        let line_tr = line.trim();
        add_line(res, line);
        if line_tr == "*/" {
            generate_code_from_config(template_path, &config, structs, plugins, input)?;
            break;
        }
        add_line(&mut config, line);
    }
    Ok(())
}

fn handle_slide_line(
    line: &str,
    lines: &mut Lines,
    template_path: &Path,
    res: &mut String,
    structs: &mut String,
    plugins: &mut String,
    input: &mut String,
) -> Result<(), Box<dyn Error>> {
    let error = |msg| Err(InvalidCommandError::new(msg).into());

    let mut parts = line.split_ascii_whitespace();
    parts.next();
    match parts.next() {
        Some("include") => {
            if let Some(include_path) = parts.next() {
                let indent = get_indentation(line);
                add_line_indented(
                    res,
                    &indent,
                    &format!("// start include from {}", include_path),
                );
                let included = get_include_content(template_path, include_path)?;
                for inc_line in included.lines() {
                    add_line_indented(res, &indent, inc_line);
                }
                add_line_indented(
                    res,
                    &indent,
                    &format!("// end include from {}", include_path),
                );
            } else {
                return error("Missing include path");
            }
        }
        Some("struct") => {
            add_line(res, line);
            res.push_str(&structs);
            add_line(res, skip_to(lines, "end_struct")?);
        }
        Some("plugin") => {
            add_line(res, line);
            res.push_str(&plugins);
            add_line(res, skip_to(lines, "end_plugin")?);
        }
        Some("input") => {
            add_line(res, line);
            let indent = get_indentation(line);
            for line in input.lines() {
                add_line_indented(res, &indent, line);
            }
            add_line(res, skip_to(lines, "end_input")?);
        }
        other => {
            match other {
                Some(value) => {
                    if value.starts_with("plugin_") {
                        add_line(res, line);
                    } else {
                        return error(value);
                    }
                }
                None => return error("No command given"),
            };
        }
    }
    Ok(())
}

pub fn transform(template: &str, template_path: &Path) -> Result<String, Box<dyn Error>> {
    let mut res = String::new();
    let mut structs = String::new();
    let mut plugins = String::new();
    let mut input = String::new();

    let mut lines = template.lines();

    while let Some(line) = lines.next() {
        let line_tr = line.trim();

        if line_tr.starts_with("/*!slide config") {
            add_line(&mut res, line);
            handle_config(
                template_path,
                &mut lines,
                &mut res,
                &mut structs,
                &mut plugins,
                &mut input,
            )?;
        } else if line_tr.starts_with("//!slide") {
            handle_slide_line(
                line,
                &mut lines,
                template_path,
                &mut res,
                &mut structs,
                &mut plugins,
                &mut input,
            )?;
        } else {
            add_line(&mut res, line);
        }
    }
    Ok(res)
}

pub fn generate(file: &Path, template_path: &Path) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file)?;
    let res = transform(&contents, template_path)?;
    fs::write(file, res)?;
    Ok(())
}
