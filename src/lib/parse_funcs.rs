use std::collections::HashMap;

use crate::lib::{
    req_res_structs::{BodyType, Method, Response},
    request::Request,
    server_errors::ServerError, // для структуры SeverError
};

// функция публичная (pub)
pub fn parse_request(req_body: String) -> Result<Request, ServerError> {
    let mut lines = req_body.lines(); // возвращает итератором по подстрокам, т.е. либо по символам 1) \n
    // либо 2) \r\n
    // &str — срез строки, представляет ссылку на участок UTF-8 в уже существующем String (или на статический литерал).
    let start_line = match lines.next() // вызываем метод .next у итератора lines
    {
        Some(line) => line, // если есть строка, то присваиваем её переменной start_line
        None             => return Err(ServerError::OtherError), // если строки нет, то возвращаем ошибку
    };

    // ------------ ЧАСТЬ №1 ------------
    // Например, теперь в start_line находится: "GET / HTTP/1.1" или "GET /path HTTP/1.1"
    // Разобьём start_line
    let mut parts = start_line.split_whitespace(); // разделим строку start_line по пробелам

    // Первый parts -- это HTTP-метод
    let method_str: &str = parts.next().unwrap_or_default();

    // Преобразуем &str в Method
    let method: Method = match method_str {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        _ => Method::OTHER, // _ это паттер, назыв wildcard (подстановочный знак)
    };

    // Второй — это путь
    let path: String = parts.next().unwrap_or_default().to_string();

    // ------------ ЧАСТЬ №2 ------------
    // Считаем все headers у сырого http запроса
    let mut headers = Vec::new(); // изменяемый вектор headers

    for line in &mut lines
    // идем итератором по строкам, разделенным \r\n
    // &mut lines даёт элемент строчки сырого запроса
    {
        let line = line.trim(); // убираем с обеих сторон любые пробельные символы (пробел, \n)
        if line.is_empty() {
            break;
        }
        headers.push(line.to_string()); // если найденная строка не пустая, конвертируем в String и кладём в вектор headers
    }

    let body_str: String = lines.collect::<Vec<&str>>().join("\r\n");

    let body: Option<BodyType> = if body_str.is_empty() {
        None // Если строка пустая, то ничего -- None
    } else {
        match serde_json::from_str::<serde_json::Value>(&body_str) {
            Ok(val) => Some(BodyType::Json(val)),
            Err(_) => Some(BodyType::Plain(body_str)),
        }
    };

    let ret_request = Request {
        method,
        path,
        headers: if headers.is_empty() {
            None
        } else {
            Some(headers)
        },
        body,
        rest_params: HashMap::new(),
    };
    // println!("{:?}", ret_request);
    Ok(ret_request) // Возвращаем успешный результат
}

fn response_code_phrase(code: usize) -> &'static str {
    match code {
        200 => "OK",
        202 => "Accepted",
        404 => "Not Found",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        502 => "Bad Gateway",
        _ => "",
    }
}

pub fn deser_response(response: Response) -> String {
    let mut http_raw_response: String = String::default(); // пустая строка
    // ------------ ЧАСТЬ №1 ------------ Формируем Status-line для http-raw-ответа
    http_raw_response += "HTTP/1.1 "; // += принимает &str, т.е. срез строки 
    // => компилятор разворачивает это в вызов http_raw_response.push_str("HTTP/1.1");
    // метод push.str копирует байты из &str (из среза) в конец буфера String
    http_raw_response += &response.response_code.to_string();
    // для usize вызываю .to_string() => получаю String
    // беру ссылку на &String, т.е. превращаю в &str => добавляю к http_raw_response
    http_raw_response += " ";
    http_raw_response += response_code_phrase(response.response_code);
    // функция возвращает &'static str, т.е. ссылку на литерал (например "OK")
    // => добавляю к http_raw_response
    http_raw_response += "\r\n"; // добавим перенос на новую строку

    // ------------ ЧАСТЬ №2 ------------ Формируем Headers для http-raw-ответа
    // if let ПАТТЕРН = ВЫРАЖЕНИЕ { … } else { … }
    let headers_from_struct: Option<Vec<String>> = if let Some(headers) =
        // получу в итоге Some(headers) или None
        response.headers
    {
        // распакова response.headers: Option<Vec<String>>
        Some(headers) // если там что-то есть, то верну Some(headers), т.е. упаковываю обратно в Some
    } else {
        None // если ничего нет, то None
    };

    if let Some(headers_vector) = headers_from_struct {
        // if let Some(...) пытается распаковать headers_from_struct
        // Если headers_from_struct == None, то весь if-блок будет пропущен
        // Если headers_from_struct == Some(vec), то vec (типа Vec<String>) переходит в переменную headers_vector
        for header in headers_vector {
            // итератор header по вектору headers_vector
            http_raw_response += &header; // добавим каждый разыменованный headers к сырому ответу
            http_raw_response += "\r\n"; // добавим перенос на новую строку
        }
    };

    // Формируем заголовок с количеством символов в body

    let body_from_struct: String = if let Some(body) = response.body {
        // если в response.body что-то есть, то это что-то присвоить body и выполнить match
        match body {
            // если body имеет тип BodyType::Plain, то
            BodyType::Plain(text) => {
                http_raw_response.push_str("Content-Type: text/plain\r\n");
                text
            }
            // если body имеет тип BodyType::Json, то
            BodyType::Json(value) => {
                let json = serde_json::to_string(&value).unwrap();
                http_raw_response.push_str("Content-Type: application/json\r\n");
                json
            }
        }
    } else {
        // выполнится, если response.body == None
        String::new()
    };

    if !(body_from_struct.len() == 0) {
        http_raw_response.push_str(&format!("Content-Length: {}\r\n", body_from_struct.len()));
        // макрос format! возвращает String -- в него можно добавить значение переменной
    }

    // ------------ ЧАСТЬ №3 ------------ Формируем CRLF - пустую строку для http-raw-ответа
    http_raw_response += "\r\n"; // добавим перенос на новую строку

    // ------------ ЧАСТЬ №4 ------------ Формируем Body для http-raw-ответа
    http_raw_response.push_str(&body_from_struct);

    http_raw_response
}
// Для запуска test нужно написать команду cargo
#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn parse_get_no_body() -> Result<(), ServerError> {
        let req_raw: String = "GET /api/status HTTP/1.1\r\n\
Host: api.example.com\r\n\
Content-Type: application/json\r\n\
\r\n"
            .to_string();

        let real_rez: Request = parse_request(req_raw)?;

        let headers: Option<Vec<String>> = Some(vec![
            "Host: api.example.com".to_string(),
            "Content-Type: application/json".to_string(),
        ]);

        let expected: Request = Request {
            method: Method::GET,
            path: "/api/status".to_string(),
            headers,
            body: None,
            rest_params: HashMap::new(),
        };

        assert_eq!(real_rez, expected);

        Ok(()) // Возвращаем 1ый аргумент из Result<(),ServerError>
    }

    #[test]
    fn parse_get_localhost() -> Result<(), ServerError> {
        let raw_reqwest = "GET / HTTP/1.1\r\n\
            Host: localhost\r\n\
            \r\n"
            .to_string();

        let real_result = parse_request(raw_reqwest)?;

        let headers = vec!["Host: localhost".to_string()];

        let expected_result = Request {
            method: Method::GET,
            path: "/".to_string(),
            headers: Some(headers),
            body: None,
            rest_params: HashMap::new(),
        };

        assert_eq!(real_result, expected_result);

        Ok(())
    }

    #[test]
    fn parse_post_json() -> Result<(), ServerError> {
        let raw_reqwest = "POST /api/users HTTP/1.1\r\n\
            Host: api.example.com\r\n\
            Content-Type: application/json\r\n\
            Content-Length: 27\r\n\
            \r\n\
            {\"name\":\"alice\",\"age\":30}"
            .to_string();

        let real_result = parse_request(raw_reqwest)?;

        let headers = vec![
            "Host: api.example.com".to_string(),
            "Content-Type: application/json".to_string(),
            "Content-Length: 27".to_string(),
        ];

        let json_body = json!({
            "name": "alice",
            "age": 30
        });

        let expected_result = Request {
            method: Method::POST,
            path: "/api/users".to_string(),
            headers: Some(headers),
            body: Some(BodyType::Json(json_body)),
            rest_params: HashMap::new(),
        };

        assert_eq!(real_result, expected_result);

        Ok(())
    }

    #[test]
    fn parse_put_json() -> Result<(), ServerError> {
        let raw_reqwest = "PUT /api/items/42 HTTP/1.1\r\n\
Host: api.example.com\r\n\
Authorization: Bearer TOKEN123\r\n\
Content-Type: application/json\r\n\
Accept: *\r\n\
\r\n\
{\"price\":19.99,\"stock\":100}"
            .to_string();

        let real_result = parse_request(raw_reqwest)?;

        let headers = vec![
            "Host: api.example.com".to_string(),
            "Authorization: Bearer TOKEN123".to_string(),
            "Content-Type: application/json".to_string(),
            "Accept: *".to_string(),
        ];

        let json_body = json!({
            "price": 19.99,
            "stock": 100
        });

        let expected_result = Request {
            method: Method::PUT,
            path: "/api/items/42".to_string(),
            headers: Some(headers),
            body: Some(BodyType::Json(json_body)),
            rest_params: HashMap::new(),
        };

        assert_eq!(real_result, expected_result);

        Ok(())
    }

    #[test]
    fn parse_delete_no_body() -> Result<(), ServerError> {
        let raw_reqwest = "DELETE /api/items/42 HTTP/1.1\r\n\
Host: api.example.com\r\n\
X-Debug-Mode: true\r\n\
\r\n"
            .to_string();

        let real_result = parse_request(raw_reqwest)?;

        let headers = vec![
            "Host: api.example.com".to_string(),
            "X-Debug-Mode: true".to_string(),
        ];

        let expected_result = Request {
            method: Method::DELETE,
            path: "/api/items/42".to_string(),
            headers: Some(headers),
            body: None,
            rest_params: HashMap::new(),
        };

        assert_eq!(real_result, expected_result);

        Ok(())
    }

    #[test]
    fn parse_get_query() -> Result<(), ServerError> {
        let raw_reqwest = "GET /search?q=rust+lang&sort=desc HTTP/2.0\r\n\
Host: www.example.com\r\n\
User-Agent: MyClient/1.0\r\n\
Accept: text/html,application/xhtml+xml\r\n\
Cookie: session=abcd1234; theme=dark\r\n\
\r\n"
            .to_string();

        let real_result = parse_request(raw_reqwest)?;

        let headers = vec![
            "Host: www.example.com".to_string(),
            "User-Agent: MyClient/1.0".to_string(),
            "Accept: text/html,application/xhtml+xml".to_string(),
            "Cookie: session=abcd1234; theme=dark".to_string(),
        ];

        let expected_result = Request {
            method: Method::GET,
            path: "/search?q=rust+lang&sort=desc".to_string(),
            headers: Some(headers),
            body: None,
            rest_params: HashMap::new(),
        };

        assert_eq!(real_result, expected_result);

        Ok(())
    }

    #[test]
    fn parse_form_request() -> Result<(), ServerError> {
        let raw_reqwest = "POST /login HTTP/1.1\r\n\
Host: auth.example.com\r\n\
Content-Type: application/x-www-form-urlencoded\r\n\
Content-Length: 29\r\n\
Cookie: mobile=true\r\n\
\r\n\
username=foo&password=bar"
            .to_string();

        let real_result = parse_request(raw_reqwest)?;

        let headers = vec![
            "Host: auth.example.com".to_string(),
            "Content-Type: application/x-www-form-urlencoded".to_string(),
            "Content-Length: 29".to_string(),
            "Cookie: mobile=true".to_string(),
        ];

        let expected_result = Request {
            method: Method::POST,
            path: "/login".to_string(),
            headers: Some(headers),
            body: Some(BodyType::Plain("username=foo&password=bar".to_string())),
            rest_params: HashMap::new(),
        };

        assert_eq!(real_result, expected_result);

        Ok(())
    }

    #[test]
    fn deser_response_test_plain_text() -> Result<(), ServerError> {
        let response: Response = Response {
            response_code: 200,
            headers: Some(vec![
                "Host: api.example.com".to_string(),
                "Location: https://example.com/new-resource".to_string(),
            ]),
            body: Some(BodyType::Plain("Hello world!".to_string())),
        };

        let real_result: String = deser_response(response);

        let expected_result: String = "HTTP/1.1 200 OK\r\n\
        Host: api.example.com\r\n\
        Location: https://example.com/new-resource\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: 12\r\n\
        \r\n\
        Hello world!"
            .to_string();

        //println!("expected result:\n{}",expected_result);
        //println!("real result:\n{}",real_result);

        assert_eq!(real_result, expected_result);

        Ok(())
    }

    #[test]
    fn deser_response_test_with_json_file() -> Result<(), ServerError> {
        let response: Response = Response {
            response_code: 200,
            headers: Some(vec![
                "Host: api.example.com".to_string(),
                "Location: https://example.com/new-resource".to_string(),
            ]),
            body: Some(BodyType::Json(json!({
                "price": 19.99,
                "stock": 100
            }))),
        };

        let real_result: String = deser_response(response);

        let expected_result: String = "HTTP/1.1 200 OK\r\n\
        Host: api.example.com\r\n\
        Location: https://example.com/new-resource\r\n\
        Content-Type: application/json\r\n\
        Content-Length: 27\r\n\
        \r\n\
        {\"price\":19.99,\"stock\":100}"
            .to_string();

        //println!("expected result:\n{}",expected_result);
        //println!("real result:\n{}",real_result);

        assert_eq!(real_result, expected_result);

        Ok(())
    }

    #[test]
    fn deser_response_test_with_no_json() -> Result<(), ServerError> {
        let response: Response = Response {
            response_code: 200,
            headers: Some(vec![
                "Host: api.example.com".to_string(),
                "Location: https://example.com/new-resource".to_string(),
            ]),
            body: None,
        };

        let real_result: String = deser_response(response);

        let expected_result: String = "HTTP/1.1 200 OK\r\n\
        Host: api.example.com\r\n\
        Location: https://example.com/new-resource\r\n\
        \r\n"
            .to_string();

        // println!("expected result:\n{}", expected_result);
        // println!("real result:\n{}", real_result);

        assert_eq!(real_result, expected_result);

        Ok(())
    }
}

/* Код который проверяет является ли body json файлом только в случае есть есть нужный заголовок content-type: application/jso
let mut is_json = false;
for header in &headers
{
    if header.to_lowercase().starts_with("content-type: application/json")
    {
        is_json = true;
    }
}

if is_json // если заявлено, что это json файл, то попробуем распарсить json
{
    match serde_json::from_str::<serde_json::Value>(&body_str)
    {
        Ok(val)  => Some(BodyType::Json(val)),
        Err(_)   => return Err(ServerError::OtherError),
    }
}
else
{
    // plain-text
    Some(BodyType::Plain(body_str))
} */

/*let method_str = match parts.next()// next() у итератора вернем Option<&str> - Option, потому что в итераторе может не оказаться больше элементов
{                                                        // если в parts есть строка (например "GET"), то вернется Some("GET")
    Some(value) => value,                          // если в parts нет строки, то вернется None и мы получим пустую строку
    None              => ""
};*/
