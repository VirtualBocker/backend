use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

use crate::lib::{
    parse_funcs::{deser_response, parse_request},
    req_res_structs::{Method, Request, Response},
    server_errors::ServerError,
};

type HandlerFn = fn(Request) -> Response;

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
    handlers: HashMap<Method, HashMap<String, HandlerFn>>,
}

impl Server {
    // Новый экземплеря сервера
    pub fn new(addr: &str) -> Result<Server, ServerError> {
        // Привязываем наш сервак на адрес "addr", чтобы он считывал
        // подключения, которые приходят на него
        // Например если "addr" будет являться чем-то типа "127.0.0.1:8080", то
        // 127.0.0.1 - айпи машины, а 8080 - порт прослушки соединения

        let listener = TcpListener::bind(addr)
            .map_err(|e| ServerError::InitError(format!("Failed to init TCP listener: {e}")))?;

        // Инициализируем нашу хешмапу которая будет хранить хендлеры
        let mut handlers: HashMap<Method, HashMap<String, HandlerFn>> = HashMap::new();

        handlers.insert(Method::GET, HashMap::new());
        handlers.insert(Method::POST, HashMap::new());
        handlers.insert(Method::PUT, HashMap::new());
        handlers.insert(Method::DELETE, HashMap::new());
        handlers.insert(Method::OTHER, HashMap::new());

        // Возвращаем наш объект сервера
        Ok(Self { listener, handlers })
    }

    // Добавляет GET хендлер на какой-то path
    #[allow(non_snake_case)]
    pub fn GET(&mut self, path: String, handler: HandlerFn) -> Result<(), ServerError> {
        let paths = self.handlers.get_mut(&Method::GET).unwrap(); // Получаем хэшмапу с путями и хэндлерами
        if paths.contains_key(&path) {
            // в хэшмапе уже есть такой путь? лови ошибку
            return Err(ServerError::HandlerError(format!(
                "GET handler with path '{path}' already registered!"
            )));
        }

        paths.insert(path, handler); // добавляем хэндер в хэшмапу по заданному пути

        Ok(())
    }

    // Добавляет POST хендлер на какой-то path
    #[allow(non_snake_case)]
    pub fn POST(&mut self, path: String, handler: HandlerFn) {
        todo!()
    }

    // Добавляет PUT хендлер на какой-то path
    #[allow(non_snake_case)]
    pub fn PUT(&mut self, path: String, handler: HandlerFn) {
        todo!()
    }

    // Добавляет DELETE хендлер на какой-то path
    #[allow(non_snake_case)]
    pub fn DELETE(&mut self, path: String, handler: HandlerFn) {
        todo!()
    }

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

                    if let Ok(request) = parse_request(raw_request) {
                        // Если получилось нормально спарсить запрос
                        if let Some(handler) = self
                            .handlers // Если нашли хэндлер в нашей хэш-таблице
                            .get(&request.method)
                            .unwrap()
                            .get(&request.path)
                        {
                            let response = deser_response(handler(request)); // Хэндлер сработал и получили ответ! Сразу его превратили в строку
                            if let Err(e) = stream.write_all(response.as_bytes()) {
                                // Отправили юзеру ответ!
                                eprintln!("Error sending response: {e}");
                            };
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
