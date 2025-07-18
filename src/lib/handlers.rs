use crate::lib::docker_works::{ContainerError, ContainerInfo, parse_docker_ps_a}; // структура для информации про один мой контейнер
use crate::lib::req_res_structs::{BodyType, Response}; // стрктура ответа
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

            let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new(); // Пустое дерево (пока что)
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
                let description_for_label = serde_json::json!({
                    "status": format!("{}",one_container.status), // преобразуем сначала в String с помощью пользовательского вывода
                    "command": one_container.command,
                    "image": one_container.image,
                });
                // вставляем пару: ключ - one_container.label и значение - description_for_label
                map.insert(one_container.label, description_for_label);
                /*  Теперь в map хранится следующее:
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

            let json_body: serde_json::Value = serde_json::Value::Object(map); // единый serde_json::Value объект
            // Оборачиваем наш Map<String, serde_json::Value> в Value::Object,
            // чтобы получить единый JSON‑объект (serde_json::Value), с которым
            // уже могут работать все функции сериализации из serde_json.
            //println!("{:?}", json_body);

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
        Err(_) => {
            let resp: Response = Response {
                // мой возвращаемый Response
                response_code: 500,
                headers: None, // заголовки Content-Type: application/json и Content-Length: {number} будут добавлены в функции deser_response
                body: None,
            };

            resp
        }
    }
}
