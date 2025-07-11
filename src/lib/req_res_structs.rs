#[derive(Debug, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: Option<Vec<String>>,
    pub body: Option<BodyType>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            method: Method::GET,
            path: String::default(),
            headers: None,
            body: None,
        }
    }
}

pub struct Response {
    response_code: usize,
    headers: Option<Vec<String>>,
    body: Option<BodyType>,
}

#[derive(Debug, PartialEq)]
pub enum BodyType {
    Json(serde_json::Value),
    Plain(String),
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    OTHER,
}
