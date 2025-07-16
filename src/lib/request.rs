use std::collections::HashMap;

use crate::lib::req_res_structs::{BodyType, Method};

#[derive(Debug, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: Option<Vec<String>>, // Option - либо Some, либо None
    pub body: Option<BodyType>,       // Option - либо Some, либо None
    pub rest_params: HashMap<String, String>
}

impl Default for Request {
    fn default() -> Self {
        Self {
            method: Method::GET,
            path: String::default(),
            headers: None,
            body: None,
            rest_params: HashMap::new()
        }
    }
}

impl Request {
    fn parse_rest_arg(arg: &str) -> String {
        String::default()
    }
}
