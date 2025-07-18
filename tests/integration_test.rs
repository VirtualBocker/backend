use std::thread::{spawn, JoinHandle};

use backend::lib::{http_server::Server, req_res_structs::{BodyType, Response}, request::Request};

const IP: &str = "127.0.0.1:8080";

fn server_start() -> JoinHandle<()> {

    let mut server = Server::new(IP).unwrap();

    // регистрация пары path и handlers в Hash-table
    server.GET("/container/", |_| {
        Response { response_code: 200, headers: None, body: Some(BodyType::Plain("GET CONTAINERS!!!".to_string())) }
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

    let response = reqwest::blocking::get(format!("http://{IP}/container/")).unwrap();

    assert_eq!(response.status().as_u16(), 200); // проверяем что ответ 200
    assert_eq!(response.text().unwrap(), String::from("GET CONTAINERS!!!")) // проверяем что тело как в хендлере
    
}

#[test]
fn post_containers_id() {


    let client = reqwest::blocking::Client::new();

    let response = client.post(format!("http://{IP}/container/256/reboot")).send().unwrap();

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.text().unwrap(), "Container ID is: 256\nRebooting...\nTEST");
}

#[test]
fn get_unknown_path() {

    let response = reqwest::blocking::get(format!("http://{IP}/unknown/path")).unwrap();

    assert_eq!(response.status().as_u16(), 404);
}