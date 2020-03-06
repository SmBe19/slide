use std::path::Path;
use std::error::Error;
use std::fs;
use std::collections::HashMap;

use crate::errors::{ConfigError, InvalidCommandError};
use std::str::{Lines, Chars};
use std::fmt;

fn get_include_content(template_path: &Path, include: &str) -> Result<String, Box<dyn Error>> {
    let path = template_path.join("include").join(format!("{}.cpp", include));
    transform(&fs::read_to_string(path)?, template_path)
}

#[derive(Debug, Clone)]
struct InpStruct {
    long: String,
    elements: Vec<String>,
}

#[derive(Debug)]
enum InpType {
    Integer,
    Float,
    String,
    Pair(Box<InpType>, Box<InpType>),
    Tuple(Vec<InpType>),
    Vector(Box<InpType>),
    Struct(InpStruct),
}

impl fmt::Display for InpType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InpType::Integer => write!(f, "long"),
            InpType::Float => write!(f, "double"),
            InpType::String => write!(f, "string"),
            InpType::Pair(t1, t2) => write!(f, "pair<{}, {}>", t1, t2),
            InpType::Tuple(types) => write!(f, "tuple<{}>", types.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")),
            InpType::Vector(typ) => write!(f, "vector<{}>", typ),
            InpType::Struct(typ) => write!(f, "{}", typ.long),
        }
    }
}

fn get_type(chars: &mut Chars, defined_structs: &HashMap<char, InpStruct>) -> Option<InpType> {
    Some(match chars.next()? {
        'i' => InpType::Integer,
        'f' => InpType::Float,
        's' => InpType::String,
        'p' => {
            let t1 = get_type(chars, defined_structs)?;
            let t2 = get_type(chars, defined_structs)?;
            InpType::Pair(Box::from(t1), Box::from(t2))
        },
        't' => {
            let num = chars.next()?.to_digit(10)?;
            let types: Vec<InpType> = (0..num)
                .map(|_x| get_type(chars, defined_structs))
                .filter(|x| x.is_some())
                .map(|x| x.unwrap()).collect();
            if num != types.len() as u32 {
                return None;
            }
            InpType::Tuple(types)
        }
        'v' => InpType::Vector(Box::from(get_type(chars, defined_structs)?)),
        other => InpType::Struct((*defined_structs.get(&other)?).clone()),
    })
}

fn handle_struct(line_tr: &str, structs: &mut String, defined_structs: &mut HashMap<char, InpStruct>) -> Result<(), ConfigError> {
    let mut chars = line_tr.chars();
    chars.next();
    let short = chars.next().ok_or(ConfigError)?;
    let mut types = Vec::new();
    while let Some(typ) = get_type(&mut chars, &defined_structs) {
        types.push(typ);
    }
    let mut parts = line_tr.split(':');
    parts.next();
    let long = parts.next().ok_or(ConfigError)?.to_string();
    let elements: Vec<String> = parts.map(|s| s.to_string()).collect();
    add_line(structs, &format!("struct {} {{", long));
    if types.len() != elements.len() {
        return Err(ConfigError);
    }
    for (typ, element) in types.iter().zip(&elements) {
        add_line(structs, &format!("  {} {};", typ, element));
    }
    add_line(structs, "};");
    defined_structs.insert(short, InpStruct { long, elements});
    Ok(())
}

fn handle_plugin(line_tr: &str, template_path: &Path, plugins: &mut String, input: &mut String) -> Result<(), Box<dyn Error>> {
    let mut parts = line_tr.split_ascii_whitespace();
    let include_path = &parts.next().ok_or(ConfigError)?[1..];
    let included = get_include_content(template_path, include_path)?;
    let mut current_dest = plugins;
    for line in included.lines() {
        let line_tr = line.trim();
        if line_tr == "//!slide plugin_input" {
            current_dest = input;
        } else {
            add_line(current_dest, line);
        }
    }

    Ok(())
}

fn read_variable(typ: InpType, variable: &str, input: &mut String, indent: &String, parts: &mut std::str::Split<char>, counter: &mut String) {
    add_line_indented(input, indent, &format!("{} {};", typ, variable));
    match typ {
        InpType::Integer | InpType::Float | InpType::String => {
            add_line_indented(input, indent, &format!("cin >> {};", variable));
        },
        InpType::Pair(_, _) => {
            add_line_indented(input, indent, &format!("cin >> {0}.first >> {0}.second;", variable));
        },
        InpType::Tuple(types) => {
            for idx in 0..types.len() {
                add_line_indented(input, indent, &format!("cin >> get<{}>({});", idx, variable));
            }
        },
        InpType::Vector(typ) => {
            let length = parts.next().unwrap_or("0");
            counter.push('i');
            let push_cmd = format!("{}.push_back(tmp_{});", variable, counter);
            add_line_indented(input, indent, &format!("for(int {0} = 0; {0} < {1}; {0}++){{", counter, length));
            let mut new_indent = indent.clone();
            new_indent.push_str("  ");
            read_variable(*typ, &format!("tmp_{}", counter), input, &new_indent, parts, counter);
            add_line_indented(input, &new_indent, &push_cmd);
            add_line_indented(input, indent, "}");
        },
        InpType::Struct(inp_struct) => {
            for element in inp_struct.elements {
                add_line_indented(input, indent, &format!("cin >> {}.{};", variable, element));
            }
        },
    }
}

fn handle_variable(line_tr: &str, input: &mut String, defined_structs: &HashMap<char, InpStruct>) -> Result<(), ConfigError> {
    for inp_config in line_tr.split_ascii_whitespace() {
        let mut parts = inp_config.split(':');
        let variable = parts.next().ok_or(ConfigError)?;
        let typ_part = parts.next().unwrap_or("i");
        let mut chars = typ_part.chars();
        let typ = get_type(&mut chars, &defined_structs).ok_or(ConfigError)?;
        read_variable(typ, variable, input, &String::from(""), &mut parts, &mut String::from(""));
    }
    Ok(())
}

fn generate_input(template_path: &Path, config: &str, structs: &mut String, plugins: &mut String, input: &mut String) -> Result<(), Box<dyn Error>> {
    let mut defined_structs = HashMap::new();
    for line in config.lines() {
        let line_tr = line.trim();
        if line_tr.starts_with('}') {
            handle_struct(line_tr, structs, &mut defined_structs)?;
        } else if line_tr.starts_with('+') {
            handle_plugin(line_tr, template_path, plugins, input)?;
        } else {
            handle_variable(line_tr, input, &defined_structs)?;
        }
    }
    Ok(())
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

fn skip_to<'a>(iter: &'a mut Lines, end: &str) -> Result<&'a str, InvalidCommandError> {
    while let Some(line) = iter.next() {
        let line_tr = line.trim();
        if line_tr.starts_with("//!slide") {
            let mut parts = line_tr.split_ascii_whitespace();
            parts.next();
            if let Some(cmd) = parts.next() {
                if cmd == end {
                    return Ok(line)
                }
            }
        }
    }
    Err(InvalidCommandError::new(&format!("Missing end for {}", end)))
}

fn handle_config(template_path: &Path, lines: &mut Lines, res: &mut String, structs: &mut String, plugins: &mut String, input: &mut String) -> Result<(), Box<dyn Error>> {
    let mut config = String::new();
    while let Some(line) = lines.next() {
        let line_tr = line.trim();
        add_line(res, line);
        if line_tr == "*/" {
            generate_input(template_path, &config, structs, plugins, input)?;
            break
        }
        add_line(&mut config, line);
    }
    Ok(())
}

fn handle_slide_line(line: &str, lines: &mut Lines, template_path: &Path, res: &mut String, structs: &mut String, plugins: &mut String, input: &mut String) -> Result<(), Box<dyn Error>> {

    let error = |msg| {
        Err(InvalidCommandError::new(msg).into())
    };

    let mut parts = line.split_ascii_whitespace();
    parts.next();
    match parts.next() {
        Some("include") => {
            if let Some(include_path) = parts.next() {
                let indent = get_indentation(line);
                add_line_indented(res, &indent, &format!("// start include from {}", include_path));
                let included = get_include_content(template_path, include_path)?;
                for inc_line in included.lines() {
                    add_line_indented(res, &indent, inc_line);
                }
                add_line_indented(res, &indent, &format!("// end include from {}", include_path));
            } else {
                return error("Missing include path");
            }
        },
        Some("struct") => {
            add_line(res, line);
            res.push_str(&structs);
            add_line(res, skip_to(lines, "endstruct")?);
        },
        Some("plugin") => {
            add_line(res, line);
            res.push_str(&plugins);
            add_line(res, skip_to(lines, "endplugin")?);
        },
        Some("input") => {
            add_line(res, line);
            let indent = get_indentation(line);
            for line in input.lines() {
                add_line_indented(res, &indent, line);
            }
            add_line(res, skip_to(lines, "endinput")?);
        },
        Some("plugin_input") => {
            add_line(res, line);
        },
        other => {
            return match other {
                Some(value) => error(value),
                None => error("No command given"),
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
            handle_config(template_path, &mut lines, &mut res, &mut structs, &mut plugins, &mut input)?;
        } else if line_tr.starts_with("//!slide") {
            handle_slide_line(line, &mut lines, template_path, &mut res, &mut structs, &mut plugins, &mut input)?;
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
