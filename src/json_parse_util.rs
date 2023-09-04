#![allow(dead_code)]
#![allow(unused_variables)]
use crate::model::{json_parse_config::ParseConfig, rust_struct::RustStruct};

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
        let mut rust_struct_list: Vec<RustStruct> = vec![];

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
            
        } else if value.is_array() {

        }

        Ok(rust_struct_list)
    }

    fn parse_json_object(&self) {

    }

    fn parse_json_array(&self) {

    }
}
