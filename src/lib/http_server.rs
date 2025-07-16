use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

use crate::lib::{
    parse_funcs::{deser_response, parse_request}, req_res_structs::{Method, Response}, request::Request, server_errors::ServerError
};

type HandlerFn = fn(&Request) -> Response;
// То есть, например, handle_home(req) принимает на вход Request и возвращает Response.

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
    handlers: HashMap<Method, HashMap<String, HandlerFn>>,
}

/*
handlers                          // HashMap<Method, …>
│
├── Method::GET ──┐               // 1‑й уровень
│                 │
│   +---------------------------+  // 2‑й уровень (HashMap<String, HandlerFn>)
│   |  "/home"    → handle_home |
│   |  "/about"   → handle_about|
│   |  "/contact" → handle_contact
│   +---------------------------+
│
├── Method::POST ─┐
│                 │
│   +---------------------------+
│   | "/submit"   → handle_submit
│   +---------------------------+
│
├── Method::PUT  ─┐
│                 │
│   +---------------------------+
│   | "/api/user" → handle_put_user
│   +---------------------------+
│
└── Method::DELETE ─┐
                  │
    +---------------------------+
    | "/api/user" → handle_del_user
    +---------------------------+

*/

impl Server {
    // Новый экземплеря сервера
    pub fn new(addr: &str) -> Result<Server, ServerError> {
        // Привязываем наш сервак на адрес "addr", чтобы он считывал
        // подключения, которые приходят на него
        // Например если "addr" будет являться чем-то типа "127.0.0.1:8080", то
        // 127.0.0.1 - айпи машины, а 8080 - порт прослушки соединения

        let listener = TcpListener::bind(addr)
            .map_err(|e| ServerError::InitError(format!("Failed to init TCP listener: {e}")))?;

        // Инициализируем нашу Hash-map таблицу, которая будет хранить handlers для различных путей
        let mut handlers: HashMap<Method, HashMap<String, HandlerFn>> = HashMap::new();

        handlers.insert(Method::GET, HashMap::new());
        handlers.insert(Method::POST, HashMap::new());
        handlers.insert(Method::PUT, HashMap::new());
        handlers.insert(Method::DELETE, HashMap::new());
        handlers.insert(Method::OTHER, HashMap::new());

        // Возвращаем наш объект сервера
        Ok(Self { listener, handlers })
    }

    pub fn add_handler(
        &mut self,
        method: Method,
        path: String,
        handler: HandlerFn,
    ) -> Result<(), ServerError> {
        let paths: &mut HashMap<String, HandlerFn> =
            self.handlers.get_mut(&method).unwrap(); // Получаем Hash-map таблицу с путями и handlers
        if paths.contains_key(&path) {
            // в Hash-map таблице уже есть такой путь? лови ошибку
            return Err(ServerError::HandlerError(format!(
                "{method:?} handler with path '{path}' already registered!"
            )));
        }

        paths.insert(path, handler); // добавляем handler в Hash-map таблицу по заданному пути

        Ok(())
    }

    #[allow(non_snake_case)]
    pub fn GET(&mut self, path: String, handler: HandlerFn) -> Result<(), ServerError> {
        self.add_handler(Method::GET, path, handler)
    }

    #[allow(non_snake_case)]
    pub fn POST(&mut self, path: String, handler: HandlerFn) -> Result<(), ServerError> {
        self.add_handler(Method::POST, path, handler)
    }

    #[allow(non_snake_case)]
    pub fn PUT(&mut self, path: String, handler: HandlerFn) -> Result<(), ServerError> {
        self.add_handler(Method::PUT, path, handler)
    }

    #[allow(non_snake_case)]
    pub fn DELETE(&mut self, path: String, handler: HandlerFn) -> Result<(), ServerError> {
        self.add_handler(Method::DELETE, path, handler)
    }

    /*
    Пусть у Hash-map таблицы есть объем в 8 едениниц, т.е.
    InnerMap (capacity = 8):

    Index  0 │  []
    Index  1 │  []
    Index  2 │  []
    Index  3 │  []
    Index  4 │  []
    Index  5 │  []
    Index  6 │  []
    Index  7 │  []

    №1.Вставка первого обработчика событий GET
    server.GET("/home".to_sting(), handler_home)?;

    1. Вычисляем hash("/home"). Пусть получилось h = 11.
    2. Берём bucket_idx = h % 8 = 11 % 8 = 3.
    3. Идём в бакет под индексом 3 и кладём туда пару ("/home", handler_home).

    Index  0 │  []
    Index  1 │  []
    Index  2 │  []
    Index  3 │  [ ("/home", handler_home) ]
    Index  4 │  []
    Index  5 │  []
    Index  6 │  []
    Index  7 │  []

    №2. Вставка второго обработчика событий GET
    server.GET("/about".to_sting(), handler_about)?;

    1. Вычисляем hash("/about"). Пусть получилось h = 1.
    2. Берём bucket_idx = h % 8 = 1 % 8 = 1.
    3. Идём в бакет под индексом 1 и кладём туда пару ("/about", handler_about).

    Index  0 │  []
    Index  1 │  [ ("/about", handler_about) ]
    Index  2 │  []
    Index  3 │  [ ("/home", handler_home) ]
    Index  4 │  []
    Index  5 │  []
    Index  6 │  []
    Index  7 │  []

    №3. Вставка третьего обработчика событий GET
    server.GET("/contact".into(), handler_contact)?;

    1. Вычисляем hash("/about"). Пусть получилось h = 3.
    2. Берём bucket_idx = h % 8 = 3 % 8 = 1.
    3. Идём в бакет под индексом 3 и кладём туда пару ("/about", handler_about). Но с коллизией

    Index  0 │  []
    Index  1 │  [ ("/about",   handler_about) ]
    Index  2 │  []
    Index  3 │  [ ("/home",    handler_home)
            └─ ("/contact", handler_contact) ]
    Index  4 │  []
    Index  4 │  []
    Index  5 │  []
    Index  6 │  []
    Index  7 │  []

    После заполнения Hash-map таблицы на 90% произойдет расширение: 8 -> 16
    + произойдет рехеширование и перераспределение элементов.


    New InnerMap (capacity = 16):

    Index  0 │  []
    Index  1 │  [ ("/about",   handler_about) ]
    Index  2 │  []
    Index  3 │  [ ("/contact", handler_contact) ]
    Index  4 │  []
    …
    Index 11 │  [ ("/home",    handler_home) ]
    …
    Index 15 │  []
     */

    /*
    // Добавляет GET handler на какой-то path
    #[allow(non_snake_case)]
    pub fn GET(&mut self, path: String, handler: HandlerFn) -> Result<(), ServerError> {
        let paths: &mut HashMap<String, fn(Request) -> Response> = self.handlers.get_mut(&Method::GET).unwrap(); // Получаем Hash-map таблицу с путями и handlers
        if paths.contains_key(&path) {
            // в Hash-map таблице уже есть такой путь? лови ошибку
            return Err(ServerError::HandlerError(format!(
                "GET handler with path '{path}' already registered!"
            )));
        }

        paths.insert(path, handler); // добавляем handler в Hash-map таблицу по заданному пути

        Ok(())
    }

    // Добавляет POST handler на какой-то path
    #[allow(non_snake_case)]
    pub fn POST(&mut self, path: String, handler: HandlerFn) -> Result<(), ServerError>{
        let paths: &mut HashMap<String, fn(Request) -> Response> = self.handlers.get_mut(&Method::GET).unwrap(); // Получаем Hash-map таблицу с путями и хэндлерами
        if paths.contains_key(&path) {
            // в Hash-map таблице уже есть такой путь? лови ошибку
            return Err(ServerError::HandlerError(format!(
                "POST handler with path '{path}' already registered!"
            )));
        }

        paths.insert(path, handler); // добавляем handler в Hash-map таблицу по заданному пути

        Ok(())
    }

    // Добавляет PUT handler на какой-то path
    #[allow(non_snake_case)]
    pub fn PUT(&mut self, path: String, handler: HandlerFn) -> Result<(),ServerError> {
        let paths: &mut HashMap<String, fn(Request) -> Response> = self.handlers.get_mut(&Method::GET).unwrap(); // Получаем Hash-map таблицу с путями и хэндлерами
        if paths.contains_key(&path) {
            // в Hash-map таблице уже есть такой путь? лови ошибку
            return Err(ServerError::HandlerError(format!(
                "PUT handler with path '{path}' already registered!"
            )));
        }

        paths.insert(path, handler); // добавляем handler в Hash-map таблицу по заданному пути

        Ok(())
    }

    // Добавляет DELETE handler на какой-то path
    #[allow(non_snake_case)]
    pub fn DELETE(&mut self, path: String, handler: HandlerFn) -> Result<(),ServerError> {
        let paths: &mut HashMap<String, fn(Request) -> Response> = self.handlers.get_mut(&Method::GET).unwrap(); // Получаем Hash-map таблицу с путями и хэндлерами
        if paths.contains_key(&path) {
            // в Hash-map таблице уже есть такой путь? лови ошибку
            return Err(ServerError::HandlerError(format!(
                "DELETE handler with path '{path}' already registered!"
            )));
        }

        paths.insert(path, handler); // добавляем handler в Hash-map таблицу по заданному пути

        Ok(())
    }
     */
    // ПОКА НИ НАДА
    // pub fn middleware<F>(&mut self, middleware: F)
    // where F: Fn(Request) -> Option<Request> {
    //     todo!()
    // }

    // Запуск сервера
    // По алгоритму:
    // 1. Получаем реквест
    // 2. Парсим его
    // 3. Ошибка парсинга? Отправляем юзеру респонс с кодом 403.
    //    Иначе
    // 4. Узнаём путь и метод
    // 5. Вызываем соответствующий хендлер на спарсенный реквест
    // 6. Получаем респонс от нашего хендлера
    // 7. Отправляем респонс клиенту
    // 8. ???
    // 9. PROFIT!!!
    pub fn start(&self) {
        // Проходимся по бесконечному итератору входящих подключений
        // Почему бесконечный? Потому-что даже когда подключения закончатся,
        // Он будет ожидать дальнейших подключений
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    // если подключение по кайфу установлено и получили поток информации
                    let bufreader = BufReader::new(&stream); // создаём буферный читатель из нашего TCP потока
                    let raw_request: String = bufreader
                        .lines() // Итерируемся по строкам буфера
                        .map(|result| result.unwrap() + "\n") // К каждой строке добавляем символ новой строки
                        .take_while(|line| !line.is_empty()) // Обрабатываем итератор, пока не встретим пустую строку
                        .collect(); // Собираем всё в тип String

                    if let Ok(mut request) = parse_request(raw_request) {
                        // Если получилось нормально спарсить запрос

                        for (key, value) in self.handlers.get(&request.method).unwrap() {
                            
                            if request.is_exact(key) {

                                request.parse_args(key);

                                let response = value(&request);

                                let deserialized_response = deser_response(response);

                                let _ = stream.write_all(deserialized_response.as_bytes());

                            }

                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to establish connection: {e}") // :(
                }
            }
        }
    }
}
