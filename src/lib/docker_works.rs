
#[derive(Debug)]
pub struct ContainerInfo // информация о контейнере
{ 
    label: String, // название контейнера NAMES
    status: ContainerStatus, // статус контейнера STATUS
    command: String, // запущенная команда COMMAND
    image: String // образ дистрибутива IMAGE
} 

#[derive(Debug)]
pub enum ContainerStatus // статус контейнера
{ 
    Exited, // Выключен
    Up, // Включен
    Created, // контейнер создан, но процесс не запущен
    Paused, // контейнер приостановлен с помощью stop
    Restarting, // Демон перезапускает контейнер согласно --restart
    Removal_in_progress, // контейнер остановлен, docker удаляет его данные
    Dead, // контейнер-зомби: процесс убит, но демон не смог корректно удалить ресурс
} 

#[derive(Debug)]
pub enum ContainerError // ошибки связанные с работой с контейнерами
{ 
    DockerError(String), // Ошибка самого докера 
    ParseError(String), // Ошибка парсинга
}

// src/lib/docker_work.rs

// Парсим все докер контейнеры на системе с помощью команды
// docker ps -a --no-trunc --format "{{.Names}}\t{{.Status}}\t{{.Image}}\t{{.Command}}"
// Произошла ошибка докера - Возвращаем ContainerError::DockerError с пояснением
// Парсим. Ошибка? Возвращаем ContainerError::ParseError
// Всё окей? Возвращаем вектор информации о контейнерах Vector<ContainerInfo>
pub fn parse_docker_ps_a() -> Result<Vec<ContainerInfo>, ContainerError>
{
    let input = r#"awesome_dewdney	Exited (130) 5 minutes ago	dockurr/windows	"/usr/bin/tini -s /root/start.sh --option --really-long-flag"
jolly_margulis	Exited (0) 6 minutes ago	ubuntu:20.04	"ls -lah /some/very/long/path/to/inspect"
MyNgnix	Up 7 minutes	nginx	"/docker-entrypoint.sh nginx -g 'daemon off;'"
no_cmd	Exited (0) 30 minutes ago	alpine:3.20	""
paused_box	Paused	ubuntu:24.04	"sleep infinity"
test-ci-runner	Up 12 seconds	gitlab/gitlab-runner:alpine	"/bin/gitlab-runner run --user=gitlab-runner --working-directory=/home/gitlab-runner"
web_front	Up 3 hours	nginx:1.25-alpine	"nginx -g 'daemon off;'"
db_back	Exited (0) 2 hours ago	postgres:16.2-alpine	"docker-entrypoint.sh postgres"
cache_srv	Exited (137) 5 minutes ago	redis:7.2	"redis-server --save 60 1 --loglevel warning"
builder	Created	golang:1.22	"/bin/sh -c 'go build -o /app/bin/myapp ./...'"
worker_long_name	Created	ruby:3.3	"/usr/bin/tini -- bash -lc 'bundle exec rake jobs:work'"
init_image	Created	busybox	"sh -c 'echo init && sleep 3600'"
analytics	Up 47 minutes	myorg/spark:3.5.0-hadoop3.3	"/opt/spark/bin/spark-class org.apache.spark.deploy.worker.Worker ..."
etl_bat	Exited (1) 10 seconds ago	python:3.12-slim-bookworm	"python /etl/run_batch.py --once"
gui_app	Up 5 minutes (healthy)	myorg/gui-app:latest	"/usr/bin/entrypoint --listen 0.0.0.0:8080"
MyNgnix	Up 12 hours	nginx	"/docker-entrypoint.sh""#.to_string();

//MyNgnix	Up 12 hours	"/docker-entrypoint.sh"	nginx
//f847d41dbcf7   nginx     "/docker-entrypoint.…"   12 hours ago   Up 12 hours   80/tcp    MyNgnix


    // let mut parts: Vec<String> = next_line.split('\t').map(str::to_string).collect(); // получим вектор строк, который разделен \t (табуляцией)
            // 1. .split возвращает итератор, который при каждом вызове метода .next возвращает срез между символами табуляции
            // 2. .map берёт каждый элемент итератор и применяет к ней функцию в ()
            // 3. .collect собирает все элементы итератора в одну коллекцию

    let mut containers:Vec<ContainerInfo> = Vec::new(); // сюда будем складывать все считанные контейнеры



    for line in input.lines()
    {
        // разобьём одну line по \t:
        let parts: Vec<&str> = line.split('\t').collect();

        // считаем label
        if parts.len() < 4
        {
            return Err(ContainerError::ParseError(format!("Parse error!\nUnexpected columns (expected 4), got {}) {}", parts.len(), line)));
        }

        // 1. Получим label
        let label:String = parts[0].to_string();

        // 2. Получим status
        let status:ContainerStatus = 
        {
            // положим в эту переменную первое слово до пробела
            let parts_status:&str = parts[1].split_whitespace().next().unwrap_or("");

            match parts_status
            {
                "Exited" => ContainerStatus::Exited,
                "Up" => ContainerStatus::Up,
                "Created" => ContainerStatus::Created,
                "Paused" => ContainerStatus::Paused,
                "Restarting" => ContainerStatus::Restarting,
                "Removal" => ContainerStatus::Removal_in_progress,
                "Dead" => ContainerStatus::Dead,
                _other =>
                {
                    return Err(ContainerError::ParseError(format!("Parse error!\nUnkown status {} in line {}",parts[2],line)));
                }
            }
        };

        // 3+4. Получим Image и Command
        let image:String = parts[2].to_string();
        let command:String = 
        {
            let command_str: &str = parts[3];
            if command_str.len() >= 2
            {
                command_str[1..command_str.len() - 1].to_string() // считываем, не включая первый и последний символ, которые являются "".
            }
            else
            {
                return Err(ContainerError::ParseError(format!("Parse error!\nUnknown command {} in line {}",parts[3], line)))
            }
        };
        // Добавим считанный контейнер в вектор контейнеров
        containers.push(ContainerInfo { label: (label), status: (status), command: (command), image: (image) });
    }
    
    //println!("{:#?}", containers);

    Ok(containers)
}

fn is_valid_label(label: &str) -> bool // прием параметра по ссылке, чтобы не передавать владение данной функции
{
    label.chars().all(|c:char| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    //.char - создаёт итератор по строке label в виде char
    //.all - метод-итератор: выполняет переданную функцию-замыкание для каждого символа
    //      true - все символы успешно прошли проверку
    //      false - хотя бы один символ провалил проверку
}

// общая функция для всех действий с docker: start, stop, pause
fn docker_action(action: &str, label: &str) -> Result<(), ContainerError>
{
    // 1. ПРОВЕРКА КОРРЕКТНОСТИ ИМЕНИ НА ЗАПРЕЩЕННЫЕ СИМВОЛЫ
    if !is_valid_label(&label) // передаю по ссылке
    {
        return Err(ContainerError::DockerError(format!("Docker Error!\nInvalid label. Use ASCII symbols, '-', '_'")));
    }

    // 2. ПРОВЕРКА НА НАЛИЧИЕ ТАКОГО КОНТЕЙНЕРА В vec<ConteinerInfo>
    let all:Vec<ContainerInfo> = parse_docker_ps_a()?; // синтаксический сахар для распаковки значений Result для проброски ошибок в помойку
    if !all.iter().any(|one_element:&ContainerInfo| one_element.label == label)
    {
        return Err(ContainerError::DockerError(format!("Docker Error!\nCan't find {} in list of containers", label)));     // или свой вариант ошибки
    }

    let output:std::process::Output = std::process::Command::new("docker")// Команда-объект, которая запускает исполняемый файл docker
        .args(&[action, &label]) // .args - добавляем аргументы для командной строки: stop + label. Итого: docker stop <label> 
        // передаем срез, &label чтобы не передавать владение, а передать ссылку + конвертация &String в &str.
        .output() // .output() блокирует текущий поток, пока процесс не будет завершен
        // output возвращает Result<_,std::io::Error>. 
        .map_err(|e:std::io::Error| ContainerError::DockerError(format!("Docker Error: {}", e)))?; // Добавил вопрос, поэтому в итоге output имеет итп std::process::Output
        // .map_err - преобразует системную ошибку в пользовательскую
        //            e - замыкание, принимающее исходную ошибку
        //            e превращается из Result<_,std::io::Error> в Result<_,ContainerError>

    if !output.status.success() // если возвращенный код не успешен, т.е. не = 0
    {
        return Err(ContainerError::DockerError(format!("Docker Error!\n{} was found. Unsucsessful attempt to stop {}",label,label )));
    }

    Ok(()) // вернем unit в случае успеха
}

// Отправляем команду остановки контейнеру с именем label
// с помощью docker stop label
// Ошибка докера? Возвращаем ошибку ContainerError::DockerError
pub fn stop_container(label: String) -> Result<(), ContainerError>  // () - unit
{
    if let Err(e) = docker_action("stop", &label)
    {
        return Err(ContainerError::DockerError(format!("Docker Error!\nCant stop container named {}\nMistake: {:?}", label, e)));
    }
    Ok(())
}

// Отправляем команду запуска контейнеру с именем label
// с помощью docker start label
// Ошибка докера? Возвращаем ошибку ContainerError::DockerError
pub fn start_container(label: String) -> Result<(), ContainerError>
{
    if let Err(e) = docker_action("start", &label)
    {
        return Err(ContainerError::DockerError(format!("Docker Error!\nCant start container named {}\nMistake: {:?}", label, e)));
    }
    Ok(())
}

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