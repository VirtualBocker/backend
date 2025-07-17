use std::collections::HashMap;

use crate::lib::req_res_structs::{BodyType, Method};

#[derive(Debug, PartialEq, Clone)]
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
    pub fn parse_args(&mut self, path: &str) {

        let request_chunks: Vec<&str> = self.path.split("/").collect();

        for (i, key_chunk) in path.split("/").enumerate() {
            if i == 0 { continue; }
            if key_chunk.starts_with(":") {
                let (_, id) = key_chunk.split_at(1);
                self.rest_params.insert(id.to_string(), request_chunks[i].to_string());
            }
        }
    }

    // ПРОВИРЯЕТ ЧТО ПУТЬ ИЗ РЕКВЕСТА И ПУТЬ ИЗ АРГУМЕНТА АНАЛОГИЧНЫ
    // НЕ СЧИТАЯ ВСЯКИХ ТАМ БЛЯТЬ АРГУМЕНТОВ
    pub fn is_exact(&mut self, path: &str) -> bool {
        let request_chunks: Vec<&str> = self.path.split("/").collect();
        for (i, key_chunk) in path.split("/").enumerate() {
            if i == 0 { continue; }
            if !key_chunk.starts_with(":") && !(key_chunk == request_chunks[i]) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_id_containers() {
        let path = "/container/:id/reboot";

        let mut request = Request {
            method: Method::GET,
            path: "/container/label/reboot".to_string(),
            headers: None,
            body: None,
            rest_params: HashMap::new(),
        };

        let mut expected_request = request.clone();
        expected_request.rest_params.insert("id".to_string(), "label".to_string());

        request.parse_args(path);
        assert!(request.is_exact(path));
        assert_eq!(request, expected_request);
    }

}