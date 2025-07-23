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

// пользовательский тип с именем DefaultResponses
#[derive(Debug)]
pub struct DefaultResponses {
    pub ok: Response,                    // 200 Ok
    pub bad_request: Response,           // 400 Bad Request
    pub not_found: Response,             // 404 Not Found
    pub conflict: Response,              // 409 Conflict
    pub internal_server_error: Response, // 500 Enternal Server Error
}

// Константный конструктор пустого Response
impl Response {
    // передаём в конструктор code -> возвращаем всю структуру Response
    const fn only_code(code: usize) -> Self {
        Response {
            response_code: code,
            headers: None,
            body: None,
        }
    }
}

// Инициализация поля response_code у const Responses:
// Создаём const DEFAULT_RESPONSES пользовательского типа DefaultResponses
// с помощью вызовов конструктора only_code
pub const DEFAULT_RESPONSES: DefaultResponses = DefaultResponses {
    ok: Response::only_code(200),                    // 200 Ok
    bad_request: Response::only_code(400),           // 400 Bad Request
    not_found: Response::only_code(404),             // 404 Not Found
    conflict: Response::only_code(409),              // 409 Conflict
    internal_server_error: Response::only_code(500), // 500 Enternal Server Error
};

/*
pub const BAD_REQUEST_RESPONSE: Response = Response {
    response_code: 400, // Bad Request (изменил: Богдан посмотри)
    headers: None,
    body: None,
};

pub const NOT_FOUND_RESPONSE: Response = Response {
    response_code: 404, // Not Found
    headers: None,
    body: None,
};

pub const INTERNAL_SERVER_ERROR_RESPONSE: Response = Response {
    response_code: 500, // Internal Server Error
    headers: None,
    body: None,
};

pub const OK_RESPONSE: Response = Response{
    response_code: 200, // Ok
    headers: None,
    body: None,
};

pub const CONFLICT_RESPONSE: Response = Response{
    response_code: 409, // Conflict
    headers: None,
    body: None,
};*/

#[derive(Debug, PartialEq, Clone)]
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
impl std::fmt::Display for Method {
    // объявляем реализацию трейта Display из модуля std::fmt
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // &self - само значение Method (GET, POST, PUT, DELETE, OTHER)
        // f: &mut fmt::Formatter<'_> -- приемник вывода. Внутри него хранятся все параметры форматирования (ширина, выравнивание, точность) + буфер, куда нужно записать результат
        // сопоставим каждый возможный self с нужным вариантов
        // fmt::Result -- это псевдоним для Result<(), std::fmt::Error>, т.е. это тоже самое. Если все успешно -- вернем Ok(()). Если ошибка - вернем Err(...).
        let stroka: &'static str = match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::OTHER => "OTHER",
        };
        // макрос write! записывает в форматер f строку s
        write!(f, "{stroka}")
    }
}
