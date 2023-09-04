#![allow(dead_code)]
#![allow(unused_variables)]

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParseConfig {
    pub serde_derive: bool,
    pub debug_derive: bool,
    pub clone_derive: bool,
    pub public_struct: bool,
    pub option: bool
}

impl ParseConfig {
    pub fn new() -> Self {
        ParseConfig {
            serde_derive: false,
            debug_derive: false,
            clone_derive: false,
            public_struct: false,
            option: false,
        }
    }

    pub fn with_params(serde_derive: bool, debug_derive: bool, clone_derive: bool, public_struct: bool, option: bool) -> Self {
        ParseConfig { serde_derive, debug_derive, clone_derive, public_struct, option }
    }
 
}