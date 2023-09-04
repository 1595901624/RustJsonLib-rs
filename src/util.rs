#![allow(dead_code)]
use crate::model::constant::KEYWORDS;

/// Judge whether the string is valid json
pub fn is_json(s: &str) -> bool {
    match serde_json::from_str::<serde_json::Value>(s) {
        Ok(_) => true,
        _ => false
    }
}

/// Judge whether the string is Numeric
pub fn is_numeric(s: &str) -> bool {
    // iterate over the string
    for c in s.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    true
}

/// Convert CamelCase to snake_case
pub fn camel_to_snake(s: &str) -> String {
    let mut result = String::new();
    for i in 0..s.len() {
        let c = s.chars().nth(i).unwrap();
        if c.is_uppercase() {
            if i != 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

/// Convert First Letter to Uppercase
pub fn capitalize_first_letter(s: &str) -> String {
    let mut result = String::new();
    for i in 0..s.len() {
        let c = s.chars().nth(i).unwrap();
        if i == 0 {
            result.push(c.to_ascii_uppercase());
        } else {
            result.push(c);
        }
    }
    result
}

/// whether the string is keyword
pub fn is_default_rust_keyword(s: &str) -> bool {
    KEYWORDS.contains(&s)
}