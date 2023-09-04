mod json_parse_util;
mod model;
mod util;

pub use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

#[cfg(test)]
mod tests {
    use crate::model::rust_field::RustField;
    use crate::model::rust_struct::RustStruct;
    use crate::model::rust_type::RustType;

    #[test]
    fn it_works() {
        let rust_struct = RustStruct::new("Root".to_string(), vec![], true, true, true, true, true);
        rust_struct.fields.borrow_mut().push(RustField::new("name".to_string(), RustType::Str, true, None));
        rust_struct.fields.borrow_mut().push(RustField::new("age".to_string(), RustType::Integer32, true, None));
        rust_struct.fields.borrow_mut().push(RustField::new("address".to_string(), RustType::Str, true, None));
        println!("{}", rust_struct.to_rust_struct_string());
    }
}
