mod json_parse_util;
mod model;
mod util;

use json_parse_util::JsonParseUtil;
use model::json_parse_config::ParseConfig;
pub use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn parse_json_default(json: &str) -> String {
    let parse_config = ParseConfig::with_params(true, true, true, true, true);
    let parse_util = JsonParseUtil::with_config(parse_config);
    let result = parse_util.parse_json(json.to_string());
    if let Ok(struct_list) = result {
        let mut struct_string = String::new();
        for ele in struct_list {
            //println!("{}", ele.to_rust_struct_string());
            struct_string.push_str(&ele.to_rust_struct_string());
        }
        return struct_string;
    }
    return "".to_string();
}

///
/// parse json by parse config
///
/// # Parameters
/// - json: json string
/// - params_js_value: params
///
pub fn parse_json(json: &str, params_js_value: JsValue) -> String {
    if let Ok(parse_config) = serde_wasm_bindgen::from_value::<ParseConfig>(params_js_value) {
        let parse_util = JsonParseUtil::with_config(parse_config);
        let result = parse_util.parse_json(json.to_string());
        if let Ok(struct_list) = result {
            let mut struct_string = String::new();
            for ele in struct_list {
                //println!("{}", ele.to_rust_struct_string());
                struct_string.push_str(&ele.to_rust_struct_string());
            }
            return struct_string;
        }
    }
    return "".to_string();
}

#[cfg(test)]
mod tests {
    use crate::json_parse_util::JsonParseUtil;
    use crate::model::json_parse_config::ParseConfig;
    use crate::model::rust_field::RustField;
    use crate::model::rust_struct::RustStruct;
    use crate::model::rust_type::RustType;

    #[test]
    fn it_works() {
        let rust_struct = RustStruct::new("Root".to_string(), vec![], true, true, true, true, true);
        rust_struct.fields.borrow_mut().push(RustField::new(
            "name".to_string(),
            RustType::Str,
            true,
            None,
        ));
        rust_struct.fields.borrow_mut().push(RustField::new(
            "age".to_string(),
            RustType::Integer32,
            true,
            None,
        ));
        rust_struct.fields.borrow_mut().push(RustField::new(
            "address".to_string(),
            RustType::Str,
            true,
            None,
        ));
        println!("{}", rust_struct.to_rust_struct_string());
    }

    #[test]
    fn test_parse_util() {
        let parse_config = ParseConfig::with_params(true, true, true, true, true);
        let parse_util = JsonParseUtil::with_config(parse_config);
        let json = r#"{
            "name": "zhangsan",
            "sex": false,
            "city": {
                "country": "beijing"
            },
            "weather": [false,true,false],
            "test":[{"x":1},{"y":1},{"z":1}],
            "age": 1
        }"#;
        // println!("{}", f64::MAX);
        let result = parse_util.parse_json(json.to_string());
        if let Ok(struct_list) = result {
            for ele in struct_list {
                println!("{}", ele.to_rust_struct_string());
            }
        } else {
            println!("{}", result.unwrap_err());
        }
    }
}
