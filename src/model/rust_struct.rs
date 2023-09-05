use std::cell::RefCell;
use serde::{Deserialize, Serialize};
use crate::model::rust_field::RustField;
use crate::model::rust_type::RustType;
use crate::util::{camel_to_snake, is_default_rust_keyword, is_numeric};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RustStruct {
    pub name: String,
    pub fields: RefCell<Vec<RustField>>,
    pub serde: bool,
    pub public: bool,
    pub option: bool,
    pub debug: bool,
    pub clone: bool,
}

impl RustStruct {
    pub fn new(name: String, fields: Vec<RustField>, serde: bool, public: bool, option: bool, debug: bool, clone: bool) -> Self {
        RustStruct {
            name,
            fields: RefCell::new(fields),
            serde,
            public,
            option,
            debug,
            clone,
        }
    }

    pub fn to_rust_struct_string(&self) -> String {
        let mut result = String::new();

        // save field name
        let mut fixed_name_vec = Vec::new();
        for field in self.fields.borrow().iter() {
            let fixed_name = self.process_field_name(field.clone());
            fixed_name_vec.push(fixed_name);
            // field.fixed_name = fixed_name.clone();
            // fixed_fields.push(field.clone());
        }

        // fix struct field name
        let mut fixed_fields: Vec<RustField> = Vec::new();
        let mut mut_fields = self.fields.borrow_mut();
        for i in 0..mut_fields.len() {
            let field = mut_fields.get_mut(i).unwrap();
            field.fixed_name = fixed_name_vec[i].clone();
            fixed_fields.push(field.clone());
        }

        // add derive
        if self.serde && self.debug && self.clone {
            result.push_str("#[derive(Serialize, Deserialize, Debug, Clone)]\n");
        } else if self.serde && self.debug {
            result.push_str("#[derive(Serialize, Deserialize, Debug)]\n");
        } else if self.serde && self.clone {
            result.push_str("#[derive(Serialize, Deserialize, Clone)]\n");
        } else if self.debug && self.clone {
            result.push_str("#[derive(Debug, Clone)]\n");
        } else if self.serde {
            result.push_str("#[derive(Serialize, Deserialize)]\n");
        } else if self.debug {
            result.push_str("#[derive(Debug)]\n");
        } else if self.clone {
            result.push_str("#[derive(Clone)]\n");
        }

        // add public
        if self.public {
            result.push_str("pub ");
        }

        // add struct name
        result.push_str(&format!("struct {} {{\n", self.name));

        // add fields
        for (index, field) in fixed_fields.iter().enumerate() {
            result.push_str("\t");

            // add serde
            if self.serde {
                result.push_str(&format!("#[serde(rename = \"{}\")]\n\t", field.name));
            }

            // add public
            if field.public {
                result.push_str("pub ");
            }

            // add field name
            result.push_str(&format!("{}: ", field.fixed_name));

            // add option
            if self.option {
                result.push_str("Option<");
            }

            // add field type
            result.push_str(&field.rust_type.to_string());

            // add object name
            match field.rust_type {
                RustType::Vec => {
                    result.push_str(&format!("<{}>", field.object_name.as_ref().unwrap_or(&"".to_string())));
                }
                RustType::Obj => {
                    result.push_str(field.object_name.as_ref().unwrap_or(&"".to_string()));
                }
                _ => {}
            }

            // add option
            if self.option {
                result.push_str(">");
            }

            result.push_str(",\n");

            // add a blank line
            if index != fixed_fields.len() - 1 {
                result.push_str("\n");
            }
        }

        // add end
        result.push_str("}\n");

        result
    }

    pub fn process_field_name(&self, field: RustField) -> String {
        let mut temp_name = camel_to_snake(&field.fixed_name);

        // if name is rust keyword, add "struct name" as prefix
        if is_default_rust_keyword(&temp_name) {
            temp_name = format!("{}_{}", camel_to_snake(self.name.as_str()), temp_name);
        }

        if is_numeric(temp_name.as_str()) {
            temp_name = format!("{}_{}", camel_to_snake(self.name.as_str()), temp_name);
        }

        // if name is duplicate, add "struct name" as prefix until not duplicate
        while self
            .fields
            .borrow()
            .iter()
            .filter(|&f| f.fixed_name == temp_name && *f != field)
            .count()
            > 0
        {
            temp_name = format!("{}_{}", camel_to_snake(self.name.as_str()), temp_name);
        }

        temp_name
    }
}

// pub fn process_field_name(cur_struct: RustStruct, field: RustField) -> String {
//     let mut temp_name = camel_to_snake(&field.fixed_name);

//     // if name is rust keyword, add "struct name" as prefix
//     if is_default_rust_keyword(&temp_name) {
//         temp_name = format!("{}_{}", camel_to_snake(cur_struct.name.as_str()), temp_name);
//     }

//     if is_numeric(temp_name.as_str()) {
//         temp_name = format!("{}_{}", camel_to_snake(cur_struct.name.as_str()), temp_name);
//     }

//     // if name is duplicate, add "struct name" as prefix until not duplicate
//     while cur_struct
//         .fields
//         .borrow_mut()
//         .iter()
//         .filter(|&f| f.fixed_name == temp_name && *f != field)
//         .count()
//         > 0
//     {
//         temp_name = format!("{}_{}", camel_to_snake(cur_struct.name.as_str()), temp_name);
//     }

//     temp_name
// }