use std::str::{Lines, SplitAsciiWhitespace};
use std::collections::HashMap;
use std::error::Error;
use crate::errors::ConfigError;
use regex::Regex;
use crate::util::{skip_to_count, add_line};
use crate::ty::{get_all_types, InpType};

pub fn handle_plugin_options<'a>(options: &mut HashMap<&'a str, &'a str>, option_keys: &mut Vec<&'a str>, lines: &'a mut Lines) -> Result<(), Box<dyn Error>> {
    while let Some(line) = lines.next() {
        let line_tr = line.trim();
        if line_tr == "*/" {
            break;
        }
        if line_tr.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line_tr.split_ascii_whitespace().collect();
        if parts.len() != 2 {
            return Err(ConfigError.into());
        }
        options.insert(parts[0], parts[1]);
        option_keys.push(parts[0]);
    }
    Ok(())
}

pub fn parse_plugin_options<'a>(options: &mut HashMap<&'a str, &'a str>, option_keys: &Vec<&'a str>, parts: &'a mut SplitAsciiWhitespace) -> Result<(), Box<dyn Error>> {
    for (idx, part) in parts.enumerate() {
        let part_tr = part.trim();
        if part_tr.starts_with("-") {
            options.insert(&part_tr[1..], "false");
        } else if part_tr.starts_with("+") {
            options.insert(&part_tr[1..], "true");
        } else {
            let parts: Vec<&str> = part.split("=").collect();
            if parts.len() != 1 && parts.len() != 2 {
                return Err(ConfigError.into());
            }
            if parts.len() == 1 {
                options.insert(option_keys.get(idx).ok_or(ConfigError)?, parts[0]);
            } else {
                options.insert(parts[0], parts[1]);
            }
        }
    }
    Ok(())
}

pub fn generate_plugin_code(options: &mut HashMap<&str, &str>, template: &str, input: &mut String, plugins: &mut String) -> Result<(), Box<dyn Error>> {
    let mut current_dest = plugins;
    let mut lines = template.lines();
    let func_regex = Regex::new(r"£([a-z]+):([a-z]+)£")?;
    let empty_hashmap = HashMap::new();
    while let Some(line) = lines.next() {
        let line_tr = line.trim();
        if line_tr.starts_with("/*!slide plugin_config") {
            while let Some(line) = lines.next() {
                let line_tr = line.trim();
                if line_tr == "*/" {
                    break;
                }
            }
        } else if line_tr.starts_with("//!slide") {
            let mut parts = line_tr.split_ascii_whitespace();
            parts.next();
            match parts.next() {
                Some("plugin_input") => {
                    if options.get("input").ok_or(ConfigError)? == &"true" {
                        current_dest = input;
                    } else {
                        break;
                    }
                },
                Some("plugin_if") => {
                    let variable = parts.next().ok_or(ConfigError)?;
                    let should_skip = if variable.starts_with("!") {
                        options.get(&variable[1..]).ok_or(ConfigError)? != &"false"
                    } else {
                        options.get(variable).ok_or(ConfigError)? != &"true"
                    };
                    if should_skip {
                        skip_to_count(&mut lines, Some("plugin_if"), "plugin_end_if")?;
                    }
                },
                Some("plugin_end_if") => (),
                Some(_) => return Err(ConfigError.into()),
                None => return Err(ConfigError.into()),
            }
        } else {
            let mut new_line = String::from(line);
            for func in func_regex.captures_iter(line) {
                let arg = func.get(2)
                    .and_then(|arg| options.get(arg.as_str()))
                    .ok_or(ConfigError)?;
                let replacement = match func.get(1).ok_or(ConfigError)?.as_str() {
                    "ty" => {
                        let types = get_all_types(&mut arg.chars(), &empty_hashmap);
                        types.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")
                    },
                    "tyvar" => {
                        let types = get_all_types(&mut arg.chars(), &empty_hashmap);
                        types.iter().enumerate().map(|(i, ty)| format!("{} v{}", ty, i)).collect::<Vec<String>>().join(", ")
                    },
                    "var" => {
                        let types = get_all_types(&mut arg.chars(), &empty_hashmap);
                        types.iter().enumerate().map(|(i, _ty)| format!("v{}", i)).collect::<Vec<String>>().join(", ")
                    },
                    "memovec" => {
                        let types = get_all_types(&mut arg.chars(), &empty_hashmap);
                        types_to_vectors(&types)?.to_string()
                    },
                    "memoresize" => {
                        let types = get_all_types(&mut arg.chars(), &empty_hashmap);
                        generate_memo_resize(0, &types)?
                    },
                    "memoacc" => {
                        let types = get_all_types(&mut arg.chars(), &empty_hashmap);
                        types.iter().enumerate().map(|(i, _ty)| format!("v{}", i)).collect::<Vec<String>>().join("][")
                    }
                    _ => return Err(ConfigError.into())
                };
                new_line = new_line.replace(func.get(0).ok_or(ConfigError)?.as_str(), &replacement);
            }
            for (k, v) in options.iter() {

                new_line = new_line.replace(&format!("${}$", k), v);
            }
            add_line(current_dest, &new_line);
        }
    }
    Ok(())
}

fn types_to_vectors(types: &[InpType]) -> Result<InpType, Box<dyn Error>> {
    if types.is_empty() || types[0] != InpType::Integer {
        return Err(ConfigError.into());
    }
    if types.len() == 1 {
        Ok(InpType::Vector(Box::from(InpType::Integer)))
    } else {
        Ok(InpType::Vector(Box::from(types_to_vectors(&types[1..])?)))
    }
}

fn generate_memo_resize(idx: i32, types: &[InpType]) -> Result<String, Box<dyn Error>> {
    if types.len() > 1 {
        let mut res = String::new();
        res.push_str(&format!("v{}", idx));
        res.push_str(", ");
        res.push_str(&types_to_vectors(&types[1..])?.to_string());
        res.push('(');
        res.push_str(&generate_memo_resize(idx+1, &types[1..])?);
        res.push(')');
        Ok(res)
    } else {
        Ok(format!("v{}", idx))
    }
}
