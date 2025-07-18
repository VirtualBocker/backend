use std::{fmt::Display, io::BufRead};

#[derive(Debug)]
pub struct ContainerInfo
// информация о контейнере
{
    pub label: String,           // название контейнера NAMES
    pub status: ContainerStatus, // статус контейнера STATUS
    pub command: String,         // запущенная команда COMMAND
    pub image: String,           // образ дистрибутива IMAGE
}

#[derive(Debug)]
pub enum ContainerStatus
// статус контейнера
{
    Exited,            // Выключен
    Up,                // Включен
    Created,           // контейнер создан, но процесс не запущен
    Paused,            // контейнер приостановлен с помощью stop
    Restarting,        // Демон перезапускает контейнер согласно --restart
    RemovalInProgress, // контейнер остановлен, docker удаляет его данные
    Dead,              // контейнер-зомби: процесс убит, но демон не смог корректно удалить ресурс
}
// реализация пользовательского вывода
impl std::fmt::Display for ContainerStatus {
    // объявляем реализацию трейта Display из модуля std::fmt
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // &self - само значение ContainerStatus
        // f: &mut fmt::Formatter<'_> -- приемник вывода. Внутри него хранятся все параметры форматирования (ширина, выравнивание, точность) + буфер, куда нужно записать результат
        // сопоставим каждый возможный self с нужным вариантов
        // fmt::Result -- это псевдоним для Result<(), std::fmt::Error>, т.е. это тоже самое. Если все успешно -- вернем Ok(()). Если ошибка - вернем Err(...).
        let stroka: &'static str = match self {
            ContainerStatus::Exited => "Exited",
            ContainerStatus::Up => "Up",
            ContainerStatus::Created => "Created",
            ContainerStatus::Paused => "Paused",
            ContainerStatus::Restarting => "Restarting",
            ContainerStatus::RemovalInProgress => "RemovalInProgress",
            ContainerStatus::Dead => "Dead",
        };
        // макрос write! записывает в форматер f строку s
        write!(f, "{stroka}")
    }
}

#[derive(Debug)]
pub enum ContainerError
// ошибки связанные с работой с контейнерами
{
    DockerError(String), // Ошибка самого докера
    ParseError(String),  // Ошибка парсинга
}

impl Display for ContainerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DockerError(val) => {
                write!(f, "Docker error: {val}")
            }
            Self::ParseError(val) => {
                write!(f, "Parse error: {val}")
            }
        }
    }
}

// src/lib/docker_work.rs

// Парсим все докер контейнеры на системе с помощью команды
// docker ps -a --no-trunc --format "{{.Names}}\t{{.Status}}\t{{.Image}}\t{{.Command}}"
// Произошла ошибка докера - Возвращаем ContainerError::DockerError с пояснением
// Парсим. Ошибка? Возвращаем ContainerError::ParseError
// Всё окей? Возвращаем вектор информации о контейнерах Vector<ContainerInfo>
pub fn parse_docker_ps_a() -> Result<Vec<ContainerInfo>, ContainerError> {
    let cmd_output: std::process::Output = std::process::Command::new("docker") // Команда-объект, которая запускает исполняемый файл docker
        .args(["ps", "-a", "--no-trunc", "--format"])
        .arg("{{.Names}}\t{{.Status}}\t{{.Image}}\t{{.Command}}") // .args - добавляем аргументы для командной строки
        .output() // .output() блокирует текущий поток, пока процесс не будет завершен
        // output возвращает Result<_,std::io::Error>.
        // output - запуск нашей команды: docker <action> <label>
        .map_err(|e: std::io::Error| ContainerError::DockerError(format!("{e}")))?; // Добавил вопрос, поэтому в итоге output имеет итп std::process::Output
    // .map_err - преобразует системную ошибку в пользовательскую
    //            e - замыкание, принимающее исходную ошибку
    //            e превращается из Result<_,std::io::Error> в Result<_,ContainerError>

    if !cmd_output.status.success()
    // если возвращенный код не успешен, т.е. не = 0
    {
        return Err(ContainerError::DockerError(format!(
            "{}",
            String::from_utf8(cmd_output.stdout).unwrap()
        )));
    }

    // let mut parts: Vec<String> = next_line.split('\t').map(str::to_string).collect(); // получим вектор строк, который разделен \t (табуляцией)
    // 1. .split возвращает итератор, который при каждом вызове метода .next возвращает срез между символами табуляции
    // 2. .map берёт каждый элемент итератор и применяет к ней функцию в ()
    // 3. .collect собирает все элементы итератора в одну коллекцию

    let mut containers: Vec<ContainerInfo> = Vec::new(); // сюда будем складывать все считанные контейнеры

    for line in cmd_output.stdout.lines() {
        // разобьём одну line по \t:

        let line = line.unwrap();

        let parts: Vec<&str> = line.split('\t').collect();

        // считаем label
        if parts.len() < 4 {
            return Err(ContainerError::ParseError(format!(
                "Unexpected columns (expected 4), got {} in {}",
                parts.len(),
                &line
            )));
        }

        // 1. Получим label
        let label: String = parts[0].to_string();

        // 2. Получим status
        let status: ContainerStatus = {
            // положим в эту переменную первое слово до пробела
            let parts_status: &str = parts[1].split_whitespace().next().unwrap_or("");

            match parts_status {
                "Exited" => ContainerStatus::Exited,
                "Up" => ContainerStatus::Up,
                "Created" => ContainerStatus::Created,
                "Paused" => ContainerStatus::Paused,
                "Restarting" => ContainerStatus::Restarting,
                "Removal" => ContainerStatus::RemovalInProgress,
                "Dead" => ContainerStatus::Dead,
                _other => {
                    return Err(ContainerError::ParseError(format!(
                        "Unkown status {} in line {}",
                        parts[2], &line
                    )));
                }
            }
        };

        // 3+4. Получим Image и Command
        let image: String = parts[2].to_string();
        let command: String = {
            let command_str: &str = parts[3];
            if command_str.len() >= 2 {
                command_str[1..command_str.len() - 1].to_string() // считываем, не включая первый и последний символ, которые являются "".
            } else {
                return Err(ContainerError::ParseError(format!(
                    "Unknown command {} in line {}",
                    parts[3], &line
                )));
            }
        };
        // Добавим считанный контейнер в вектор контейнеров
        containers.push(ContainerInfo {
            label: (label),
            status: (status),
            command: (command),
            image: (image),
        });
    }

    //println!("{:#?}", containers);

    Ok(containers)
}

fn is_valid_label(label: &str) -> bool // прием параметра по ссылке, чтобы не передавать владение данной функции
{
    label
        .chars()
        .all(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    //.char - создаёт итератор по строке label в виде char
    //.all - метод-итератор: выполняет переданную функцию-замыкание для каждого символа
    //      true - все символы успешно прошли проверку
    //      false - хотя бы один символ провалил проверку
}

// общая функция для всех действий с docker: start, stop, pause
fn docker_action(action: &str, label: &str) -> Result<(), ContainerError> {
    // 1. ПРОВЕРКА КОРРЕКТНОСТИ ИМЕНИ НА ЗАПРЕЩЕННЫЕ СИМВОЛЫ
    if !is_valid_label(label)
    // передаю по ссылке
    {
        return Err(ContainerError::DockerError(
            "Invalid label. Use ASCII symbols, '-', '_'".to_string(),
        ));
    }

    // 2. ПРОВЕРКА НА НАЛИЧИЕ ТАКОГО КОНТЕЙНЕРА В vec<ConteinerInfo>
    let all: Vec<ContainerInfo> = parse_docker_ps_a()?; // синтаксический сахар для распаковки значений Result для проброски ошибок в помойку
    if !all
        .iter()
        .any(|one_element: &ContainerInfo| one_element.label == label)
    {
        return Err(ContainerError::DockerError(format!(
            "Can't find {label} in list of containers"
        ))); // или свой вариант ошибки
    }

    let output: std::process::Output =
        std::process::Command::new("docker") // Команда-объект, которая запускает исполняемый файл docker
            .args([action, label]) // .args - добавляем аргументы для командной строки: stop + label. Итого: docker stop <label>
            // передаем срез, &label чтобы не передавать владение, а передать ссылку + конвертация &String в &str.
            .output() // .output() блокирует текущий поток, пока процесс не будет завершен
            // output возвращает Result<_,std::io::Error>.
            // output - запуск нашей команды: docker <action> <label>
            .map_err(|e: std::io::Error| ContainerError::DockerError(format!("{e}")))?; // Добавил вопрос, поэтому в итоге output имеет итп std::process::Output
    // .map_err - преобразует системную ошибку в пользовательскую
    //            e - замыкание, принимающее исходную ошибку
    //            e превращается из Result<_,std::io::Error> в Result<_,ContainerError>

    if !output.status.success()
    // если возвращенный код не успешен, т.е. не = 0
    {
        return Err(ContainerError::DockerError(format!(
            "{label} was found. Unsucsessful attempt to stop {label}"
        )));
    }

    Ok(()) // вернем unit в случае успеха
}

// Отправляем команду остановки контейнеру с именем label
// с помощью docker stop label
// Ошибка докера? Возвращаем ошибку ContainerError::DockerError
pub fn stop_container(label: String) -> Result<(), ContainerError> // () - unit
{
    docker_action("stop", &label)
}

// Отправляем команду запуска контейнеру с именем label
// с помощью docker start label
// Ошибка докера? Возвращаем ошибку ContainerError::DockerError
pub fn start_container(label: String) -> Result<(), ContainerError> {
    docker_action("start", &label)
}

// ПЕРЕДЕЛАТЬ В ТЕСТЫ!!!!!!!!!!!

/*

   // ------------ DOCKER_WORKS.RS РАЗДЕЛ ------------

    let containers: Result<Vec<ContainerInfo>, ContainerError> = parse_docker_ps_a();

    println!("{:#?}", containers);

    let mut rezult = stop_container("MyNgnix".to_string());

    match rezult {
        Ok(()) => println!("Container sucessfully stopped."),
        Err(_err) => println!("Docker Error! Failed to stop container!"),
    }

    rezult = start_container("MyNgnix".to_string());

    match rezult {
        Ok(()) => println!("Container sucessfully started."),
        Err(_err) => println!("Docker Error! Failed to start container!"),
    }

*/

// Название ошибки не пишется не пишется -- убрать DockerError
/*

СТОИТ ТАКЖЕ ПРОВЕРЯТЬ label НА ПРАВИЛЬНОСТЬ. А ТО НЕДОХАКЕР ВСТАВИТ
КОМАНДУ КАКУЮ-НИБУДЬ В label И ПОЛУЧИТСЯ ЧТО-ТО ТИПА

docker stop label1 && rm -rf /

ЕСЛИ label = "label1 && rm -rf /"

Вариант 1 фикса:
    есть спецсимволы - возвращаем ContainerError
Вариант 2 фикса:
    получаем все контейнеры, если запрошенный label (параметр функции) не совпадает ни с одним из вектора всех контейнеров, возвращаем ContainerError

*/
