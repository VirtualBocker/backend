use crate::lib::{
    req_res_structs::{Request, Response},
    server_errors::ServerError,
};

struct Server {
    // listener: TCPListener,
}

impl Server {
    // Новый экземплеря сервера
    pub fn new() -> Server {
        todo!()
    }

    // Добавляет GET хендлер на какой-то path
    #[allow(non_snake_case)]
    pub fn GET<F>(&mut self, path: String, handler: F)
    where
        F: Fn(Request) -> Response,
    {
        todo!()
    }

    // Добавляет POST хендлер на какой-то path
    #[allow(non_snake_case)]
    pub fn POST<F>(&mut self, path: String, handler: F)
    where
        F: Fn(Request) -> Response,
    {
        todo!()
    }

    // Добавляет PUT хендлер на какой-то path
    #[allow(non_snake_case)]
    pub fn PUT<F>(&mut self, path: String, handler: F)
    where
        F: Fn(Request) -> Response,
    {
        todo!()
    }

    // Добавляет DELETE хендлер на какой-то path
    #[allow(non_snake_case)]
    pub fn DELETE<F>(&mut self, path: String, handler: F)
    where
        F: Fn(Request) -> Response,
    {
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
    fn start(&mut self) -> Result<(), ServerError> {
        Err(ServerError::OtherError)
    }
}
