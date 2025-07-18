use std::collections::HashMap;

use crate::lib::req_res_structs::{BodyType, Method};

#[derive(Debug, PartialEq, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: Option<Vec<String>>, // Option - либо Some, либо None
    pub body: Option<BodyType>,       // Option - либо Some, либо None
    pub rest_params: HashMap<String, String>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            method: Method::GET,
            path: String::default(),
            headers: None,
            body: None,
            rest_params: HashMap::new(),
        }
    }
}

impl Request {
    // /container/:id/reboot разбивается на ["", "container", ":id", "reboot"]
    // функция для парсинга (т.е. разбиения) пути по составляющим
    pub fn parse_args(&mut self) {
        let request_chunks: Vec<&str> = self.path.trim_start_matches('/').split('/').collect();
        // /container/:id/reboot разбивается на [ container, :id, reboot ] т.к. убрали начальное / .trim_start_matches
        for (i, &key_chunk) in request_chunks.iter().enumerate() {
            // .iter - получаю итератор, .enumerate - добавляю текущему итератору counter (счетчик)
            if key_chunk.starts_with(":") {
                // подходит key_chunk = ":id"
                let (_, id) = key_chunk.split_at(1); // получаем: ":" и "id" (":" выкидываем т.к. первый аргумент _)
                self.rest_params
                    .insert(id.to_string(), request_chunks[i].to_string()); // вставляем в Hash-table<String, String> элемент <1, id>
            }
        }
    }

    /*
    pub fn parse_args(&mut self) {
        let mut request_chunks: Vec<&str> = self.path.split("/").collect(); // /container/:id/reboot разбивается на ["", "container", ":id", "reboot"]
        request_chunks.remove(0); // убираю пустой сегмент "", имеем => ["container", ":id", "reboot"]

        for (i, key_chunk) in request_chunks.into_iter().enumerate() {
            // .iter - получаю итератор, .enumerate - добавляю текущему итератору counter (счетчик)
            if key_chunk.starts_with(":") {
                // подходит key_chunk = ":id"
                let (_, id) = key_chunk.split_at(1); // получаем: ":" и "id" (":" выкидываем т.к. первый аргумент _)
                self.rest_params
                    .insert(id.to_string(), request_chunks[i].to_string()); // вставляем в Hash-table<String, String> элемент <1, id>
            }
        }
        /*
        for (i, key_chunk) in path.split("/").enumerate() {
            if key_chunk.starts_with(":") {
                let (_, id) = key_chunk.split_at(1);
                self.rest_params
                    .insert(id.to_string(), request_chunks[i].to_string());
            }
        }*/
    }*/

    // ПРОВИРЯЕТ ЧТО ПУТЬ ИЗ РЕКВЕСТА И ПУТЬ ИЗ АРГУМЕНТА АНАЛОГИЧНЫ
    // НЕ СЧИТАЯ ВСЯКИХ ТАМ АРГУМЕНТОВ
    pub fn is_similar(&mut self, path: &str) -> bool {
        let request_chunks: Vec<&str> = self.path.split("/").collect();
        let key_chunks: Vec<&str> = path.split("/").collect();

        if request_chunks.len() != key_chunks.len() {
            return false;
        };

        for (i, key_chunk) in key_chunks.iter().enumerate() {
            if !key_chunk.starts_with(":") && (*key_chunk != request_chunks[i]) {
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
        expected_request
            .rest_params
            .insert("id".to_string(), "label".to_string());


        request.parse_args(path);
        assert!(request.is_similar(path));
        assert_eq!(request, expected_request);
    }
}
