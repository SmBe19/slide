use std::collections::HashMap;
use std::fmt;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub struct InpStruct {
    pub long: String,
    pub elements: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum InpType {
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
            InpType::Integer => write!(f, "long long int"),
            InpType::Float => write!(f, "double"),
            InpType::String => write!(f, "string"),
            InpType::Pair(t1, t2) => write!(f, "pair<{}, {}>", t1, t2),
            InpType::Tuple(types) => write!(f, "tuple<{}>", types.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")),
            InpType::Vector(typ) => write!(f, "vector<{}>", typ),
            InpType::Struct(typ) => write!(f, "{}", typ.long),
        }
    }
}

pub fn get_type(chars: &mut Chars, defined_structs: &HashMap<char, InpStruct>) -> Option<InpType> {
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

pub fn get_all_types(chars: &mut Chars, defined_structs: &HashMap<char, InpStruct>) -> Vec<InpType> {
    let mut types = Vec::new();
    while let Some(typ) = get_type(chars, &defined_structs) {
        types.push(typ);
    }
    types
}
