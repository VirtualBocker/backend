use crate::lib::config;

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

use crate::lib::{
    logger::Logger,
    parse_funcs::{deser_response, parse_request},
    req_res_structs::{Method, Response},
    request::Request,
    server_errors::ServerError,
};

type HandlerFn = fn(&Request) -> Response;
// То есть, например, handle_home(req) принимает на вход Request и возвращает Response.

const BAD_REQUEST_RESPONSE: Response = Response {
    response_code: 404,
    headers: None,
    body: None,
};

const NOT_FOUND_RESPONSE: Response = Response {
    response_code: 404,
    headers: None,
    body: None,
};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
    handlers: HashMap<Method, HashMap<&'static str, HandlerFn>>,
    pub log: Logger,
    pub config: config::Config
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
    pub fn with_config(config: config::Config) -> Result<Server, ServerError> {
        let log = Logger::with_config(&config);

        let addr = "127.0.0.1:".to_string() + &config.port;

        let listener = TcpListener::bind(addr.clone())
            .map_err(|e| ServerError::InitError(format!("Failed to init TCP listener: {e}")))?;

        // Инициализируем нашу Hash-map таблицу, которая будет хранить handlers для различных путей
        let mut handlers: HashMap<Method, HashMap<&str, HandlerFn>> = HashMap::new();

        handlers.insert(Method::GET, HashMap::new());
        handlers.insert(Method::POST, HashMap::new());
        handlers.insert(Method::PUT, HashMap::new());
        handlers.insert(Method::DELETE, HashMap::new());
        handlers.insert(Method::OTHER, HashMap::new());


        // Возвращаем наш объект сервера
        Ok(Self {
            listener,
            handlers,
            log,
            config: config::Config {
                port : addr,
                ..config
            }
        })
    }

    // Новый экземпляр сервера
    pub fn new(addr: &str) -> Result<Server, ServerError> {
        // Привязываем наш сервак на адрес "addr", чтобы он считывал
        // подключения, которые приходят на него
        // Например если "addr" будет являться чем-то типа "127.0.0.1:8080", то
        // 127.0.0.1 - айпи машины, а 8080 - порт прослушки соединения
        

        // Инициализация логгера
        let log = Logger::default();
        
        let listener = TcpListener::bind(addr)
            .map_err(|e| ServerError::InitError(format!("Failed to init TCP listener: {e}")))?;

        // Инициализируем нашу Hash-map таблицу, которая будет хранить handlers для различных путей
        let mut handlers: HashMap<Method, HashMap<&str, HandlerFn>> = HashMap::new();

        handlers.insert(Method::GET, HashMap::new());
        handlers.insert(Method::POST, HashMap::new());
        handlers.insert(Method::PUT, HashMap::new());
        handlers.insert(Method::DELETE, HashMap::new());
        handlers.insert(Method::OTHER, HashMap::new());


        // Возвращаем наш объект сервера
        Ok(Self {
            listener,
            handlers,
            log,
            config:config::Config::default()
        })
    }

    pub fn add_handler(
        &mut self,
        method: Method,
        path: &'static str,
        handler: HandlerFn,
    ) -> Result<(), ServerError> {
        let paths: &mut HashMap<&str, HandlerFn> = self.handlers.get_mut(&method).unwrap(); // Получаем Hash-map таблицу с путями и handlers
        if paths.contains_key(&path) {
            // в Hash-map таблице уже есть такой путь? лови ошибку
            self.log.info(&format!(
                "{method} handler with path '{path}' already registered!"
            ));
            return Err(ServerError::HandlerError(format!(
                "{method} handler with path '{path}' already registered!"
            )));
        }

        paths.insert(path, handler); // добавляем handler в Hash-map таблицу по заданному пути
        self.log
            .info(&format!("📌 Handler registered: {method} {path}"));
        Ok(())
    }

    #[allow(non_snake_case)]
    pub fn GET(&mut self, path: &'static str, handler: HandlerFn) {
        self.add_handler(Method::GET, path, handler).unwrap()
    }

    #[allow(non_snake_case)]
    pub fn POST(&mut self, path: &'static str, handler: HandlerFn) {
        self.add_handler(Method::POST, path, handler).unwrap()
    }

    #[allow(non_snake_case)]
    pub fn PUT(&mut self, path: &'static str, handler: HandlerFn) {
        self.add_handler(Method::PUT, path, handler).unwrap()
    }

    #[allow(non_snake_case)]
    pub fn DELETE(&mut self, path: &'static str, handler: HandlerFn) {
        self.add_handler(Method::DELETE, path, handler).unwrap()
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
        self.log.motd();
        self.log.info(&"Server started".to_string());

        // Проходимся по бесконечному итератору входящих подключений
        // Почему бесконечный? Потому-что даже когда подключения закончатся,
        // Он будет ожидать дальнейших подключений
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    // если подключение по кайфу установлено и получили поток информации
                    let bufreader = BufReader::new(&stream); // создаём буферный читатель из нашего TCP потока

                    /*
                    cap -- сколько байт сейчас лежит в буфере
                    pos -- картека, индекс следующего байта в данном диапазоне

                    BufReader<R> хранит в себе следующие компоненты:

                    1. Внутренний ридер (inner: R)
                    2. Буфер (buf: Vec<u8>)
                    3. Индексы состояния (pos: usize и cap: usize)

                    Внутренний ридер -- это источник информации. Если в буфере пусто, тогда cap = 0, а pos = 0. Т.к. pos>=cap , я запрошу информацию от внутреннего ридера. При этом он оценит количество байт и это будет моё новое значение cap (т.е. я могу не заполнить весь буфер)

                    Кареткой я буду считывать до тех пор, пока вновь не выполнится pos>=cap.
                    BufReader
                    ├─ inner: TcpStream { … }
                    ├─ buf: Vec<u8> (capacity 8192)
                    └─ [raw]: (pos: 0, cap: 0)*/
                    let raw_request: String = bufreader
                        .lines() // возвращает итератор по строкам из буферизированного читателя (ридера)
                        // итератор выдает элементы типа Result<String, std::io::Error>
                        // удаляет символ /n. Если перед ним был /r тоже удаляет
                        .map(|result| result.unwrap() + "\r\n") // кратко: к каждой строке добавляем символ новой строки ("\r\n")
                        /* подробно: result.unwrap() извлекает String из Ok(String) или ломается, если при чтении произошел Error
                        т.е. распаковываем успешный результат чтения строки + добавляем \r\n, т.к. .lines убирает \r\n
                        в итоге получаем после .map преобразование итератора:
                        Исходный итератор был: Iterator<Item = Result<String, std::io::Error>>
                        В результате стал: Iterator<Item = String> */
                        .take_while(|line| !line.trim_end().is_empty()) // Обрабатываем итератор, пока не встретим пустую строку
                        // т.е. берем строки из итератора, пока не встретим пустую строку, т.е. CRLF-строку в raw-HTTP request (т.е. пока не встретим \r\n, которая означает конец заголовков)
                        // take_while останавливает итерацию, когда встретится пустая строка "". Сама пустая строка в результат не попадает
                        // + .take_while не изменяет строки
                        .collect(); // Собираем всё в тип String
                    // собирает все оставшиеся элементы итератора и скеивает их в контейнер нужного типа (String, т.к. мы его явно задали при let raw_request: String)
                    // у нас остаётся \r\n в конце каждой строки, т.к. мы вернули эту последовательность в map, а .take_while не изменяет строки
                    // println!("{}",raw_request);
                    match parse_request(raw_request) {
                        // Если получилось нормально спарсить запрос
                        Ok(mut request) => {
                            let mut found_path = false;

                            for (key, value) in self.handlers.get(&request.method).unwrap() {
                                if request.is_similar(key) {
                                    request.parse_args(key);

                                    let response = value(&request);

                                    self.log.info(&format!(
                                        "Handler triggered for route: {} {}",
                                        request.method, request.path
                                    ));

                                    let deserialized_response = deser_response(response);

                                    let _ = stream.write_all(deserialized_response.as_bytes());
                                    found_path = true;
                                    break;
                                }
                            }

                            if !found_path {
                                let _ =
                                    stream.write_all(deser_response(NOT_FOUND_RESPONSE).as_bytes());
                            }
                        }
                        Err(e) => {
                            self.log.debug(&format!("Server error: {e}"));
                            let _ =
                                stream.write_all(deser_response(BAD_REQUEST_RESPONSE).as_bytes());
                        }
                    }
                }
                Err(e) => {
                    self.log
                        .warn(&format!("Failed to establish connection: {e}")); // :)
                }
            }
        }
    }
}
