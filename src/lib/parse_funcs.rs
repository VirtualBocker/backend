use crate::lib::{
    req_res_structs::{BodyType, Method, Request, Response}, // для структур Request, Response, BodyType
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

    let body_str = lines.collect::<Vec<&str>>().join("\r\n");

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
    };

    Ok(ret_request) // Возвращаем успешный результат
}

pub fn deser_response(response: Response) -> String {
    String::default()
}

// Для запуска test нужно написать команду cargo 
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn first_test() -> Result<(),ServerError>
    {
        let req_raw:String = "GET /api/status HTTP/1.1\r\n\
Host: api.example.com\r\n\
Content-Type: application/json\r\n\
\r\n"
            .to_string();

        let real_rez:Request = parse_request(req_raw)?; 

        let headers:Option<Vec<String>> = Some(vec![
            "Host: api.example.com".to_string(),
            "Content-Type: application/json".to_string(),
        ]);

        let expected:Request = Request
        {
            method: Method::GET,
            path: "/api/status".to_string(),
            headers,
            body: None,
        };

        assert_eq!(real_rez,expected);

        Ok(()) // Возвращаем 1ый аргумент из Result<(),ServerError>

    }
    #[test]
    #[should_panic] // если в тесте Паника рассматривается как ожидаемый результат
    fn example_test()
    {
        panic!("MY PANIC")
    }
} 

// ПЕРЕДЕЛАТЬ В ТЕСТЫ
/* 
let raw_requests = vec![
        "GET /api/status HTTP/1.1\r\n\
Host: api.example.com\r\n\
Content-Type: application/json\r\n\
\r\n"
            .to_string(),
        "GET / HTTP/1.1\r\n\
Host: localhost\r\n\
\r\n"
            .to_string(),
        "POST /api/users HTTP/1.1\r\n\
Host: api.example.com\r\n\
Content-Type: application/json\r\n\
Content-Length: 27\r\n\
\r\n\
{\"name\":\"alice\",\"age\":30}"
            .to_string(),
        "PUT /api/items/42 HTTP/1.1\r\n\
Host: api.example.com\r\n\
Authorization: Bearer TOKEN123\r\n\
Content-Type: application/json\r\n\ */
//Accept: *\r\n\
/*\r\n\
{\"price\":19.99,\"stock\":100}"
            .to_string(),
        "DELETE /api/items/42 HTTP/1.1\r\n\
Host: api.example.com\r\n\
X-Debug-Mode: true\r\n\\
r\n"
        .to_string(),
        "GET /search?q=rust+lang&sort=desc HTTP/2.0\r\n\
Host: www.example.com\r\n\
User-Agent: MyClient/1.0\r\n\
Accept: text/html,application/xhtml+xml\r\n\
Cookie: session=abcd1234; theme=dark\r\n\
\r\n"
            .to_string(),
        "POST /login HTTP/1.1\r\n\
Host: auth.example.com\r\n\
Content-Type: application/x-www-form-urlencoded\r\n\
Content-Length: 29\r\n\
Cookie: mobile=true\r\n\
\r\n\
username=foo&password=bar"
            .to_string(),
    ];

    // 1 - нет
    // 2 - нет
    // 3 - да (Json)
    // 4 - да (Json)
    // 5 - нет
    // 6 - нет
    // 7 - да (Plain)

    for raw in &raw_requests {
        match parse_request(raw.clone()) {
            Ok(req) => println!("OK: {:?}", req),
            Err(err) => eprintln!("Parse error: {:?}", err),
        }
    }*/


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

