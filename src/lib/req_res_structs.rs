#[derive(Debug, PartialEq)]
pub struct Response {
    // при формировании экземпляра структуры не надо указывать:
    // 1. заголовок Content-Type: text/plain\r\n или Content-Type: application/json\r\n
    // 2. заголовок Content-Length: {number}
    // все это формируется в функции deser_response в зависимости от типа body
    pub response_code: usize,
    pub headers: Option<Vec<String>>,
    pub body: Option<BodyType>,
}

#[derive(Debug, PartialEq)]
pub enum BodyType {
    Json(serde_json::Value),
    Plain(String),
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    OTHER,
}

pub enum WordResposeCode {
    OK,       // для 200
    Created,  // для 201
    Accepted, // для 202
    NotFound, // для 404
}
