use std::hash::{Hash, Hasher};
use crate::model::rust_type::RustType;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub struct RustField {
    pub(crate) name: String,
    pub(crate) rust_type: RustType,
    pub(crate) public: bool,
    pub(crate) fixed_name: String,
    pub(crate) object_name: Option<String>,
}

impl RustField {
    pub fn new(name: String, rust_type: RustType, public: bool, object_name: Option<String>) -> Self {
        RustField {
            name: name.clone(),
            rust_type,
            public,
            fixed_name: name,
            object_name,
        }
    }

    // fn calculate_hash(&self) -> u64 {
    //     let prime = 31;
    //     let mut result = 1;
    //     let mut hasher = DefaultHasher::new();
    //     self.name.hash(&mut hasher);
    //     result = prime * result + hasher.finish();
    //     self.rust_type.hash(&mut hasher);
    //     result = prime * result + hasher.finish();
    //     self.public.hash(&mut hasher);
    //     result = prime * result + hasher.finish();
    //     if self.object_name.is_some() {
    //         self.object_name.hash(&mut hasher);
    //         result = prime * result + hasher.finish();
    //     }
    //     result
    // }
}