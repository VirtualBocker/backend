use crate::lib::docker_works::{ContainerError, ContainerInfo, parse_docker_ps_a};
use crate::lib::logger::Logger;
// структура для информации про один мой контейнер
use crate::lib::req_res_structs::{BodyType, DEFAULT_RESPONSES, Response}; // стрктура ответа
use crate::lib::request::Request; // структура запроса
use serde_json;

pub fn handler_return_all_containers(_request: &Request) -> Response {
    // Нужно обработать request и вернуть Response
    // Данный handler должен возвращать весь вектор ContainerInfo

    // parse_docker_ps_a возвращает Result<Vec<ContainerInfo>, ContainerError>
    let result: Result<Vec<ContainerInfo>, ContainerError> = parse_docker_ps_a();

    match result {
        Ok(all_my_containers) => {
            //println!("{:?}", all_my_containers);

            let mut arr: Vec<serde_json::Value> = Vec::new(); // Пустой вектор
            // let mut arr: serde_json::Map<String, serde_json::Value> = serde_json::Map::new(); // Пустое дерево (пока что)
            // под капотом это отсортированное по ключам дерево.
            // Ключ - String. Значение - любой JSON-тип: Null, Bool, Number, String, Array, Object.
            // это встроенная структура данных, которая представляет из себя JSON-объект в виде пары: ключ - значение.
            /*  serde_json::Value - перечисление (enum), которое описывает любой JSON-тип
            pub enum Value {
                Null,
                Bool(bool),
                Number(Number),
                String(String),
                Array(Vec<Value>),
                Object(Map<String, Value>),
            }
            */
            for one_container in all_my_containers {
                // итератор по вектору с контейнерами
                let description_for_label: serde_json::Value = serde_json::json!({ // это один объект типа serde_json:Value
                    "name": one_container.label,                  // вносим имя (name, он же label)
                    "status": format!("{}",one_container.status), // преобразуем сначала в String с помощью пользовательского вывода
                    "command": one_container.command,             // вносим command
                    "image": one_container.image,                 // вносим image
                });

                arr.push(description_for_label);
                /* ------ КОММЕНТАРИЙ ДЛЯ ARR (МАССИВА)  ------
                Теперь в arr хранится следующее:
                [
                    {
                        "command": "docker-entrypoint.sh mysqld",
                        "image": "mysql",
                        "name": "BD-mysql",
                        "status": "Up"
                    },
                    {
                        "command": "/docker-entrypoint.sh nginx -g 'daemon off;'",
                        "image": "nginx",
                        "name": "web1",
                        "status": "Up"
                    }
                ]
                */

                /* ------ КОММЕНАТРИЙ ДЛЯ MAP (ДЕРЕВА) ------
                Теперь в map хранится следующее:
                "analytics": {
                    "command": "/opt/spark/bin/spark-class org.apache.spark.deploy.worker.Worker ...",
                    "image": "myorg/spark:3.5.0-hadoop3.3",
                    "status": "Up"
                },
                "awesome_dewdney": {
                    "command": "/usr/bin/tini -s /root/start.sh --option --really-long-flag",
                    "image": "dockurr/windows",
                    "status": "Exited"
                },
                */
            }

            let json_body: serde_json::Value = serde_json::Value::Array(arr); // единый serde_json::Value объект (для arr)

            /*  let json_body: serde_json::Value = serde_json::Value::Object(map); // единый serde_json::Value объект (для map)
            // Оборачиваем наш Map<String, serde_json::Value> в Value::Object,
            // чтобы получить единый JSON‑объект (serde_json::Value), с которым
            // уже могут работать все функции сериализации из serde_json. */
            println!("{json_body:?}");

            // подсчиатем количество символов в serde_json::Value
            //let string: String = serde_json::to_string(&json_body).expect("serde_json::to_string error"); // чтобы отбросить ошибку

            //println!("{}", string);

            let resp: Response = Response {
                // мой возвращаемый Response
                response_code: 200,
                headers: None, // заголовки Content-Type: application/json и Content-Length: {number} будут добавлены в функции deser_response
                body: Some(BodyType::Json(json_body)),
            };

            resp
        }
        Err(e) => {
            let logger = Logger::default();
            logger.error(&format!("Container error: {e}"));
            DEFAULT_RESPONSES.internal_server_error // 500
        }
    }
}

// функция ищет совпадение имени контейнера и имени контейнеров в HashMap table
fn check_existence_container(container_id: &str) -> Response {
    // получим результат работы функции parse_docker_ps_a в виде Result<Vec<ContainerInfo>, ContainerError>
    let rezult: Result<Vec<ContainerInfo>, ContainerError> = parse_docker_ps_a();

    // деструктуризация enum rezult
    match rezult {
        Ok(all_my_containers) => {
            // если вернулся список контейнеров
            let mut flag: bool = false; // флаг: false -- не нашлось совпадений в HashMap table, true - совпадение найдено
            for one_container in all_my_containers {
                // итератор по всем контейнерам, который вернулись из функции parse_docker_ps_a()
                if one_container.label.as_str() == container_id {
                    // сравниваем два &str: если они равны, то мы нашли искомый контейнер в HashMap table
                    flag = true; // true - совпадение найдено
                    break; // выходим из цикла
                }
            }

            if !flag {
                // если не нашли, то выведем сообщение через Logger и вернем Response 500 (Internal Server Error)
                let logger = Logger::default();
                logger.error(&format!("Can't find container {container_id}:"));
                DEFAULT_RESPONSES.internal_server_error // 500
            } else {
                // если flag == true, то вернем Response 200 (Ok)
                DEFAULT_RESPONSES.ok // 200 -- Ok (нашли)
            }
        }
        Err(e) => {
            // если вернулась ошибка ContainerError, то выведем сообщение об ошибке через Logger и вернем Response 500 (Internal Server Error)
            let logger = Logger::default();
            logger.error(&format!("Container error: {e}"));
            DEFAULT_RESPONSES.internal_server_error // 500
        }
    }
}

// функция получаем id контейнера из Request
fn get_container_id(request: &Request) -> Result<&str, Response> {
    request
        .rest_params // обращаемся к полю .rest_params => берем HashMap<String, String> с параметрами пути
        .get("id") // запрашиваем значение по ключу "id" => возвращаем Option<&String>
        .map(|s| s.as_str()) // если получено Some(&String), то применим функцию .as_str и вернем полученный результат в оболочке: Option<&str>
        .ok_or_else(|| {
            // Если получено None, то вызовем замыкание => результат замыкания будет передан в Error в Result (в данном случае Err(Response))
            // Лямда-функция (замыкание)
            // 1. Выведем сообщение в logger
            let logger = Logger::default();
            logger.warn(&format!(
                "Failed to find container_id {}!",
                request.rest_params.get("id").unwrap()
            ));
            // 2. Отправим Response
            DEFAULT_RESPONSES.bad_request // 400
        })
}

/*Общий вид:
    "State": {
        "Status": "exited",
        "Running": false,
        "Paused": false,
        "Restarting": false,
        "OOMKilled": false,
        "Dead": false,
        "Pid": 0,
        "ExitCode": 0,
        "Error": "",
        "StartedAt": "2025-07-22T15:26:10.849191622Z",
        "FinishedAt": "2025-07-23T04:51:01.571691759Z"
    },
*/
// Структура куда будут считываться данные из утилиты docker
// Команда: "docker inspect <label>"
#[derive(Default)]
#[allow(dead_code)] // поле is_paused на данный момент не зайдествовано, но данные считываются
struct ReadStatus {
    status: String, // Status: running, exited, created, paused, restarting, removing, dead
    is_running: bool, // Запущен: true/false
    is_paused: bool, // Находится на паузе: true/false
    is_restarting: bool, // В процессе перезапуск: true/false
    is_dead: bool,  // Мертв: true/false
}

// Реализация конструктора default для объекта типа ReadStatus => можно заменить атрибутом Default
/*
impl Default for ReadStatus {
    fn default() -> Self {
        // не принимает параметры и возвращает себя
        Self {
            status: String::default(), // Пустая строка для Status
            is_running: false,         // для остальных полей по умолчанию false
            is_paused: false,
            is_restarting: false,
            is_dead: false,
        }
    }
}*/

// Создаём экземпляр структуры ReadStatus и заполняем его данными
fn fill_struct_read_status(container_id: &str) -> Result<ReadStatus, Response> {
    // ------------------------------------------------------------------
    // ------ №1. Запускаем docker inspect ------------------------------
    // ------------------------------------------------------------------
    let inspect: Result<std::process::Output, std::io::Error> =
        std::process::Command::new("docker")
            .arg("inspect") // positional agumets: для осмотра контейнера (выводит всю возможную информацию)
            .arg("-f") // short flag: format = в каком формате искать
            .arg("{{.State.Status}} {{.State.Running}} {{.State.Paused}} {{.State.Restarting}} {{.State.Dead}}") // значение для опции -f: вернёт поле State.Running
            .arg(container_id)
            .output();

    // Распакуем inspect (деструктуризация):
    let inspect_output = match inspect {
        Ok(out) => out,
        Err(_e) => {
            return Err(DEFAULT_RESPONSES.internal_server_error); // 500
        }
    };
    // ------------------------------------------------------------------
    // ------ №2. Преобразуем stdout в String и разбиваем по пробелам ---
    // ------------------------------------------------------------------
    //let raw = &inspect_output.stdout;          // &[u8]
    let state_str: String = String::from_utf8_lossy(&inspect_output.stdout).into_owned();
    // &inspect_output.stdout -- обращаюсь к памяти, в которой хранится std::process::Output
    // from_utf8_lossy -- вернет Result<String, FromUtf8Error>
    // .into_owned() -- Извлекает данные (String)

    let mut parts: std::str::SplitWhitespace<'_> = state_str.split_whitespace(); // итератор по подстрокам, разделенным пробелами
    //          .slit_whitespace() -- разобьём String по пробелам
    //          все итераторы по умолчанию «сидят» перед первым элементом.
    //          поэтому первый вызов `parts.next()` вернёт первый фрагмент строки.

    // ------------------------------------------------------------------
    // ------ №3. Наполняем ReadStatus ------------------------------------
    // ------------------------------------------------------------------
    let new_data: ReadStatus = ReadStatus {
        // Переменная для чтения данных в ReadStatus => заполним её с помощью конструктора default()
        status: parts.next().unwrap().to_string(),
        // .next() возвращает Option<&str> т.е. следующая подстрока из split_whitespace() (в обертке Option)
        // .unwrap_or()
        is_running: parts
            .next() // .next() -- возвращает Option<&str> т.е. следующая подстрока из split_whitespace() (в обертке Option)
            .unwrap_or("false") // .unwrap_or("false") -- подставим false, если Option::None
            .parse::<bool>() // .parse::<bool>() -- конвертируем true/false в bool (возвращает Result<bool, ParseBoolError>)
            .unwrap_or(false), // .unwrap_or(false) -- если .parse вернула error, то будем использовать false по умолчанию
        is_paused: parts
            .next()
            .unwrap_or("false")
            .parse::<bool>()
            .unwrap_or(false),
        is_restarting: parts
            .next()
            .unwrap_or("false")
            .parse::<bool>()
            .unwrap_or(false),
        is_dead: parts
            .next()
            .unwrap_or("false")
            .parse::<bool>()
            .unwrap_or(false),
    };
    Ok(new_data)
}

// Функция для выполнения докер команды
// Команда: docker <команда> <label>
//     т.е. docker <word_in_present_simple> <container_id>
fn do_docker_command(
    container_id: &str,           // label контейнера
    word_in_present_simple: &str, // слово, которое будет использовано для <команда>
    word_in_past_simple: &str, // слово, которое будет использовано для logger, чтобы сообщить об успешном выполнении команды
) -> Response {
    let docker_start: Result<std::process::Output, std::io::Error> =
        std::process::Command::new("docker")
            .arg(word_in_present_simple) // аргумент для stop контейнера
            .arg(container_id) // аргумент для имени контейнера, который stop
            .output(); // .output() блокирует текущий поток, пока процесс не будет завершен
    // output возвращает Result<_,std::io::Error>.
    // output - запуск нашей команды: docker <action> <label>

    // Деструктуризация enum для docker_start
    match docker_start {
        Ok(_smth) => {
            // Если возвращено Ok(), то
            // 1. отправим сообщение в логгер об успешном выполнении команды docker
            let logger: Logger = Logger::default();
            logger.info(&format!(
                "Sucessfully {word_in_past_simple} container {container_id}!",
            ));
            // 2. Вернем из функции Reponse Ok
            DEFAULT_RESPONSES.ok // Ok, 200 (успешно stopped)
        }
        Err(e) => {
            // Если возвращена Err(), то
            // 1. отправим сообщение в логгер об ошибке выполнении команды docker
            let logger: Logger = Logger::default();
            logger.error(&format!(
                "Failed to {word_in_present_simple} container {container_id}: {e}",
            ));
            // 2. Вернем из функции Reponse Internal Server Error
            DEFAULT_RESPONSES.internal_server_error // 500
        }
    }
}

// функция-handler, которая отрабатывает команду docker: docker start <label>
pub fn handler_start_container(request: &Request) -> Response {
    // Нужно передать команду на start контейнеру с определенным label = :id
    // :id хранится в request в поле rest_params (это Hash-table)

    // ------------------------------------------------------------------
    // ------ №1. Получим id контейнера из request ----------------------
    // ------------------------------------------------------------------
    let container_id = match get_container_id(request) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // ------------------------------------------------------------------
    // ------ №2. Проверим, что такой id (<label>) существует------------
    // ------------------------------------------------------------------
    let resp_check: Response = check_existence_container(container_id);
    if resp_check == DEFAULT_RESPONSES.internal_server_error {
        // 500
        return resp_check;
    }

    // ------------------------------------------------------------------
    // ------ №3. Проверим статусы контейнера ---------------------------
    // ------------------------------------------------------------------

    let my_data: ReadStatus = match fill_struct_read_status(container_id) {
        Ok(data) => data,
        Err(resp) => return resp,
    };

    // ------------------------------------------------------------------
    // ------ №4. Обработаем интересующие статусы контейнера ------------
    // ------------------------------------------------------------------

    /* Что сделает `docker start`:
    + 1. `Dead = true`       ⇒ контейнер «мертв», поднять его не получится → 409 Conflict
    + 2. `Restarting = true` ⇒ контейнер уже в состоянии запуска/перезапуска → 409 Conflict
    + 3. `Running = true`    ⇒ контейнер уже запущен → 409 Conflict */

    if my_data.is_dead {
        let logger: Logger = Logger::default();
        logger.warn(&format!(
            "Failed to start container {container_id}. It is dead!"
        ));
        return DEFAULT_RESPONSES.conflict; // 409
    }

    if my_data.is_restarting {
        let logger: Logger = Logger::default();
        logger.warn(&format!(
            "Failed to start container {container_id}. It is restarting!"
        ));
        return DEFAULT_RESPONSES.conflict; // 409
    }

    if my_data.is_running {
        let logger: Logger = Logger::default();
        logger.warn(&format!(
            "Failed to start container {container_id}. It is alredy running! Use restart instead!"
        ));
        return DEFAULT_RESPONSES.conflict; // 409
    }

    // ------------------------------------------------------------------
    // ------ №5. Выполним команду docker <command> <label> -------------
    // ------------------------------------------------------------------
    do_docker_command(container_id, "start", "started")
    // все остальные случаи: просто выполняем команду docker start <label>
}

// функция-handler, которая отрабатывает команду docker: docker stop <label>
pub fn handler_stop_container(request: &Request) -> Response {
    // Нужно передать команду stop контейнеру с определенным label = :id
    // :id хранится в request в поле rest_params (это Hash-table)

    // ------------------------------------------------------------------
    // ------ №1. Получим id контейнера из request ----------------------
    // ------------------------------------------------------------------
    let container_id = match get_container_id(request) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // ------------------------------------------------------------------
    // ------ №2. Проверим, что такой id (<label>) существует------------
    // ------------------------------------------------------------------
    let resp_check: Response = check_existence_container(container_id);
    if resp_check == DEFAULT_RESPONSES.internal_server_error {
        // 500
        return resp_check;
    }

    // ------------------------------------------------------------------
    // ------ №3. Проверим статусы контейнера ---------------------------
    // ------------------------------------------------------------------

    let my_data: ReadStatus = match fill_struct_read_status(container_id) {
        Ok(data) => data,
        Err(resp) => return resp,
    };

    // ------------------------------------------------------------------
    // ------ №4. Обработаем интересующие статусы контейнера ------------
    // ------------------------------------------------------------------

    /* Что сделает `docker stop`:
    1. `Dead = true`  → контейнер «мертв», остановить нельзя → вернёт ошибку “Container is not running” → 409 Conflict
    2. `Status = "exited"` или `Created`  → контейнер уже остановлен (или только создан), `docker stop` вернёт ту же ошибку → 409 Conflict
    Сам справится:
    3. `Restarting = true` → прерывает цикл рестартов, шлёт SIGTERM (и по таймауту SIGKILL) → переводит контейнер в `exited` → 200 OK
    4. `Paused = true` → автоматически выполняет unpause, затем SIGTERM (и по таймауту SIGKILL) → 200 OK
    5. `Running = true` → шлёт SIGTERM (и по таймауту SIGKILL) → 200 OK */

    if my_data.status.starts_with("exited") {
        let logger: Logger = Logger::default();
        logger.warn(&format!(
            "Failed to stop container {container_id}. It is already stopped!"
        ));
        return DEFAULT_RESPONSES.conflict; // 409
    }

    if my_data.is_dead {
        let logger: Logger = Logger::default();
        logger.warn(&format!(
            "Failed to stop container {container_id}. It is dead!"
        ));
        return DEFAULT_RESPONSES.conflict; // 409
    }

    // ------------------------------------------------------------------
    // ------ №5. Выполним команду docker <command> <label> -------------
    // ------------------------------------------------------------------

    // все остальные случаи: просто выполняем команду docker stop <label>
    do_docker_command(container_id, "stop", "stopped")
}

// функция-handler, которая отрабатывает команду docker: docker restart <label>
pub fn handler_restart_container(request: &Request) -> Response {
    // Нужно передать команду stop контейнеру с определенным label = :id
    // :id хранится в request в поле rest_params (это Hash-table)

    // ------------------------------------------------------------------
    // ------ №1. Получим id контейнера из request ----------------------
    // ------------------------------------------------------------------
    let container_id = match get_container_id(request) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // ------------------------------------------------------------------
    // ------ №2. Проверим, что такой id (<label>) существует------------
    // ------------------------------------------------------------------
    let resp_check: Response = check_existence_container(container_id);
    if resp_check == DEFAULT_RESPONSES.internal_server_error {
        // 500
        return resp_check;
    }

    // ------------------------------------------------------------------
    // ------ №3. Проверим статусы контейнера ---------------------------
    // ------------------------------------------------------------------

    let my_data: ReadStatus = match fill_struct_read_status(container_id) {
        Ok(data) => data,
        Err(resp) => return resp,
    };

    // ------------------------------------------------------------------
    // ------ №4. Обработаем интересующие статусы контейнера ------------
    // ------------------------------------------------------------------

    /* Что сделает docker при restart:
    1. Dead = true => docker restart не поднимет данный контейнер (409)
    2. Restarting = true => выдать сообщение, что контейнер уже перезагружается (409)
    Docker сам справится с:
    3. Running = true => docker restart сделает stop контейнер, затем restart.
    4. Paused = true => docker restart сделает unpaused, затем restart.
    5. Exited/Created => docker restart сделает start.*/
    if my_data.is_dead {
        let logger: Logger = Logger::default();
        logger.warn(&format!(
            "Failed to restart container {container_id}. It is dead!"
        ));
        return DEFAULT_RESPONSES.conflict; // 409
    }

    if my_data.is_restarting {
        let logger: Logger = Logger::default();
        logger.warn(&format!(
            "Failed to restart container {container_id}. It is restarting!"
        ));
        return DEFAULT_RESPONSES.conflict; // 409
    }
    // ------------------------------------------------------------------
    // ------ №5. Выполним команду docker <command> <label> -------------
    // ------------------------------------------------------------------

    // все остальные случаи: просто выполняем команду docker restart <label>
    do_docker_command(container_id, "restart", "restarted")
}
