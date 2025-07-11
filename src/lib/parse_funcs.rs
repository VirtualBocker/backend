
use crate::lib::
{
    req_res_structs::{Request, Response, BodyType, Method}, // для структур Request, Response, BodyType
    server_errors::ServerError}; // для структуры SeverError








// функция публичная (pub)
pub fn parse_request(req_body: String) -> Result<Request, ServerError> 
{
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
    let method_str = match parts.next()// next() у итератора вернем Option<&str> - Option, потому что в итераторе может не оказаться больше элементов
    {                                                        // если в parts есть строка (например "GET"), то вернется Some("GET")
        Some(value) => value,                          // если в parts нет строки, то вернется None и мы получим пустую строку
        None              => ""
    };
    // Преобразуем &str в Method
    let method: Method = match method_str
    {
        "GET"    => Method::GET,
        "POST"   => Method::POST,
        "PUT"    => Method::PUT,
        "DELETE" => Method::DELETE,
        _        => Method::OTHER, // _ это паттер, назыв wildcard (подстановочный знак)
    };

    // Второй — это путь
    let path = match parts.next()
    {
        Some(value) => value.to_string(),  // конвертация &str в String
        None              => String::new(),      // создание новой пустой строки
    };

    // ------------ ЧАСТЬ №2 ------------
    // Считаем все headers у сырого http запроса
    let mut headers = Vec::new(); // изменяемый вектор headers

    for line in &mut lines // идем итератором по строкам, разделенным \r\n
    // &mut lines даёт элемент строчки сырого запроса
    { 
        let line = line.trim(); // убираем с обеих сторон любые пробельные символы (пробел, \n)
        if line.is_empty()
        {
            break;
        }
        headers.push(line.to_string()); // если найденная строка не пустая, конвертируем в String и кладём в вектор headers
    }

    let body_str = lines.collect::<Vec<&str>>().join("\r\n");

    let body: Option<BodyType> = if body_str.is_empty()
    {
        None // Если строка пустая, то ничего -- None
    }
    else
    {
        match serde_json::from_str::<serde_json::Value>(&body_str)
        {
            Ok(val)  => Some(BodyType::Json(val)),
            Err(_)   => Some(BodyType::Plain(body_str)),
        }
    };

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

    let ret_request = Request{
        method,
        path,
        headers: if headers.is_empty() { None } else { Some(headers) },
        body,
    };

    Ok(ret_request) // Возвращаем успешный результат
}












fn deser_response(response: Response) -> String {
    String::default()
}