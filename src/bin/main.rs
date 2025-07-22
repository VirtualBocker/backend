
// Файл, который лежит в src/bin/*.rs образует crate-исполняемый файл (main)
// Из main можно пользоваться только тем, что выставлено наружу (pub) из библиотечного crate и корректно объявлено в lib.rs
use backend::lib::handlers::{
    handler_restart_container, handler_return_all_containers, handler_start_container,
    handler_stop_container,
};
use backend::lib::http_server::Server;
// use backend::lib::req_res_structs::{BodyType, Response};
// use backend::lib::request::Request;

fn main() {


    let mut ip_port = "127.0.0.1:".to_string();
    {
        let env_port = std::env::var("PORT").unwrap_or("8080".to_string());
        ip_port.push_str(env_port.as_str());
    }
    let mut server = Server::new(ip_port.as_str()).unwrap();

    server.log.debug(&format!("ip_port = {ip_port}"));

    // регистрация пары path и handlers в Hash-table
    server.GET("/container/", handler_return_all_containers); // 2ой аргумент это тип HandlerFn

    server.POST("/container/:id/restart", handler_restart_container);
    server.POST("/container/:id/start", handler_start_container);
    server.POST("/container/:id/stop", handler_stop_container);

    // server.POST("/container/:id/reboot", |r: &Request| Response {
    //     response_code: 200,
    //     headers: None,
    //     body: Some(BodyType::Plain(format!(
    //         "Container ID is: {}\nRebooting...\nTEST",
    //         r.rest_params.get("id").unwrap()
    //     ))),
    // });

    // server.POST("/container/:id/start", |r: &Request| Response {
    //     response_code: 200,
    //     headers: None,
    //     body: Some(BodyType::Plain(format!(
    //         "Container ID is: {}\nStarting...\nTEST",
    //         r.rest_params.get("id").unwrap()
    //     ))),
    // });

    // server.POST("/container/:id/stop", |r: &Request| Response {
    //     response_code: 200,
    //     headers: None,
    //     body: Some(BodyType::Plain(format!(
    //         "Container ID is: {}\nStopping...\nTEST",
    //         r.rest_params.get("id").unwrap()
    //     ))),
    // });

    server.start();
}

/* Как сейчас выглядит Hash-table
handlers                          // HashMap<Method, …>
│
├── Method::GET ──┐     // 1‑й уровень
│                 │
│   +------------------------------------------------+  // 2‑й уровень (HashMap<String, HandlerFn>)
│   |  "/container/" → handler_return_all_containers |
│   +------------------------------------------------+
│
├── Method::POST ─┐     // 1‑й уровень
│                 │
│   +---------------------------------------------+ // 2‑й уровень (HashMap<String, HandlerFn>)
│   | "/container/:id/restart" → to_be_determined  |
│   | "/container/:id/start" → to_be_determined   |
│   | "/container/:id/stop" → to_be_determined    |
│   +---------------------------------------------+
│
├── Method::PUT  ─┐
│                 │
│   +---------------------------+
│   | EMPTY                     |
│   +---------------------------+
│
└── Method::DELETE ─┐
                  │
    +---------------------------+
    | EMPTY                     |
    +---------------------------+

*/
