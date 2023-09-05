#![allow(dead_code)]
#![allow(unused_variables)]

use std::vec;

use crate::{model::{
    json_parse_config::ParseConfig, rust_field::RustField, rust_struct::RustStruct,
    rust_type::RustType,
}, util};

pub struct JsonParseUtil {
    pub parse_config: ParseConfig,
}

impl JsonParseUtil {
    pub fn new() -> Self {
        JsonParseUtil {
            parse_config: ParseConfig {
                serde_derive: false,
                debug_derive: false,
                clone_derive: false,
                public_struct: false,
                option: false,
            },
        }
    }

    pub fn with_config(parse_config: ParseConfig) -> Self {
        JsonParseUtil { parse_config }
    }

    pub fn parse_json(&self, json: String) -> Result<Vec<RustStruct>, Box<dyn std::error::Error>> {
        let mut struct_list: Vec<RustStruct> = vec![];

        // create a root struct
        let rust_struct = RustStruct::new(
            "Root".to_string(),
            vec![],
            self.parse_config.serde_derive,
            self.parse_config.public_struct,
            self.parse_config.option,
            self.parse_config.debug_derive,
            self.parse_config.clone_derive,
        );

        let value = serde_json::from_str::<serde_json::Value>(&json)?;

        if value.is_object() {
            self.parse_json_object(value.clone(), "Root".to_string(), &mut struct_list)
        } else if value.is_array() {}

        Ok(struct_list)
    }

    /// parse json object
    /// @param value
    /// @param struct_name
    /// @param struct_list 
    /// @return Vec<RustStruct>
    fn parse_json_object(
        &self,
        value: serde_json::Value,
        struct_name: String,
        struct_list: &mut Vec<RustStruct>,
    ) {
        // let name = if struct_name.is_empty() {
        //     "Root".to_string()
        // } else {
        //     struct_name.clone()
        // }
        let rust_struct = RustStruct::new(
            if struct_name.is_empty() {
                "Root".to_string()
            } else {
                struct_name.clone()
            },
            vec![],
            self.parse_config.serde_derive,
            self.parse_config.public_struct,
            self.parse_config.option,
            self.parse_config.debug_derive,
            self.parse_config.clone_derive,
        );

        let json_object = value.as_object().unwrap_or(&serde_json::Map::new()).clone();
        println!("json_object: {:?}", json_object);
        for (key, value) in json_object {
            // 字符串
            if value.is_string() {
                let rust_field = RustField::new(
                    key.clone(),
                    RustType::Str,
                    self.parse_config.public_struct,
                    None,
                );
                rust_struct.fields.borrow_mut().push(rust_field);
            } else if value.is_boolean() {
                let rust_field = RustField::new(
                    key.clone(),
                    RustType::Bool,
                    self.parse_config.public_struct,
                    None,
                );
                rust_struct.fields.borrow_mut().push(rust_field);
            } else if value.is_number() {
                if value.is_i64() {
                    let n = value.as_i64().unwrap_or(0);
                    // i32
                    if (n as i32) as i64 == n {
                        let rust_field = RustField::new(
                            key.clone(),
                            RustType::Integer32,
                            self.parse_config.public_struct,
                            None,
                        );
                        rust_struct.fields.borrow_mut().push(rust_field);
                    } else {
                        // i64
                        let rust_field = RustField::new(
                            key.clone(),
                            RustType::Integer64,
                            self.parse_config.public_struct,
                            None,
                        );
                        rust_struct.fields.borrow_mut().push(rust_field);
                    }
                } else if value.is_f64() {
                    // f64
                    let rust_field = RustField::new(
                        key.clone(),
                        RustType::Float64,
                        self.parse_config.public_struct,
                        None,
                    );
                    rust_struct.fields.borrow_mut().push(rust_field);
                } else {
                    // default i32
                    let rust_field = RustField::new(
                        key.clone(),
                        RustType::Integer32,
                        self.parse_config.public_struct,
                        None,
                    );
                    rust_struct.fields.borrow_mut().push(rust_field);
                }
            } else if value.is_object() {
                let rust_field = RustField::new(
                    key.clone(),
                    RustType::Obj,
                    self.parse_config.public_struct,
                    Some(util::capitalize_first_letter(&key))
                );
                rust_struct.fields.borrow_mut().push(rust_field);
                self.parse_json_object(value, util::capitalize_first_letter(&key), struct_list);
            } else if value.is_array() {

            } else  {
               // if value is null, as it is an empty string 
                let rust_field = RustField::new(
                    key.clone(),
                    RustType::Str,
                    self.parse_config.public_struct,
                    None
                );
                rust_struct.fields.borrow_mut().push(rust_field);
            }
        }
        struct_list.push(rust_struct);
    }

    fn parse_json_array(&self) {}
}
