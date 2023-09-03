use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub enum RustType {
    Str,
    Integer32,
    Integer64,
    Integer128,
    Float32,
    Float64,
    UnsignedInteger32,
    UnsignedInteger64,
    UnsignedInteger128,
    IntegerSize,
    UnsignedIntegerSize,
    Bool,
    Vec,
    Obj,
}

impl Display for RustType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_type())
    }
}

impl RustType {
    pub fn get_type(&self) -> &str {
        match self {
            RustType::Str => "String",
            RustType::Integer32 => "i32",
            RustType::Integer64 => "i64",
            RustType::Integer128 => "i128",
            RustType::Float32 => "f32",
            RustType::Float64 => "f64",
            RustType::UnsignedInteger32 => "u32",
            RustType::UnsignedInteger64 => "u64",
            RustType::UnsignedInteger128 => "u128",
            RustType::IntegerSize => "isize",
            RustType::UnsignedIntegerSize => "usize",
            RustType::Bool => "bool",
            RustType::Vec => "Vec",
            RustType::Obj => ""
        }
    }
}