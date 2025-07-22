use std::thread::{JoinHandle, spawn};

use backend::lib::{
    config::Config,
    http_server::Server,
    req_res_structs::{BodyType, Response},
    request::Request,
    server_errors::ServerError,
};

const IP: &str = "127.0.0.1:8080";

fn server_start() -> JoinHandle<Result<(), ServerError>> {
    let config = Config::default().with_port(8080);

    let mut server = Server::with_config(config).unwrap();

    // регистрация пары path и handlers в Hash-table
    server.GET("/container/", |_| Response {
        response_code: 200,
        headers: None,
        body: Some(BodyType::Plain("GET CONTAINERS!!!".to_string())),
    }); // 2ой аргумент это тип HandlerFn

    server.POST("/container/:id/reboot", |r: &Request| Response {
        response_code: 200,
        headers: None,
        body: Some(BodyType::Plain(format!(
            "Container ID is: {}\nRebooting...\nTEST",
            r.rest_params.get("id").unwrap()
        ))),
    });

    spawn(move || server.start()) // запускаем сервер в отдельном потоке
}

#[test]
fn get_containers() {
    let _ = server_start();

    let response = minreq::get(format!("http://{IP}/container/"))
        .send()
        .unwrap();

    assert_eq!(response.status_code, 200); // проверяем что ответ 200
    assert_eq!(
        response.as_str().unwrap(),
        String::from("GET CONTAINERS!!!")
    ) // проверяем что тело как в хендлере
}

#[test]
fn post_containers_id() {
    let response = minreq::post(format!("http://{IP}/container/256/reboot"))
        .send()
        .unwrap();

    assert_eq!(response.status_code, 200);
    assert_eq!(
        response.as_str().unwrap(),
        "Container ID is: 256\nRebooting...\nTEST"
    );
}

#[test]
fn get_unknown_path() {
    let response = minreq::get(format!("http://{IP}/unknown/path"))
        .send()
        .unwrap();
    assert_eq!(response.status_code, 404);
}
