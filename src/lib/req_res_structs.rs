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
