pub struct Request {
    method: Method,
    path: String,
    headers: Option<Vec<String>>,
    body: Option<BodyType>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            method: Method::GET,
            path: String::default(),
            headers: None,
            body: None
        }
    }
}

pub struct Response {
    response_code: usize,
    headers: Option<Vec<String>>,
    body: Option<BodyType>
}

pub enum BodyType {
    Json(serde_json::Value),
    Plain(String)
}

pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    OTHER
}