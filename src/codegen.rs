use std::collections::HashMap;

use crate::errors::ConfigError;
use crate::ty::{InpStruct, InpType, get_all_types};
use crate::util::{add_line, add_line_indented};

pub fn read_variable(typ: InpType, variable: &str, input: &mut String, indent: &String, parts: &mut std::str::Split<char>, counter: &mut String) {
    match typ {
        InpType::Integer => add_line_indented(input, indent, &format!("IN({});", variable)),
        InpType::Vector(ty) if *ty == InpType::Integer => {
            let length = parts.next().unwrap_or("0");
            add_line_indented(input, indent, &format!("INV({}, {});", variable, length));
        },
        _ => read_varialbe_fallback(typ, variable, input, indent, parts, counter),
    }
}

pub fn read_varialbe_fallback(typ: InpType, variable: &str, input: &mut String, indent: &String, parts: &mut std::str::Split<char>, counter: &mut String) {
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

pub fn struct_definition(line_tr: &str, structs: &mut String, defined_structs: &mut HashMap<char, InpStruct>) -> Result<(), ConfigError> {
    let mut chars = line_tr.chars();
    chars.next();
    let short = chars.next().ok_or(ConfigError)?;
    let types = get_all_types(&mut chars, defined_structs);
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
