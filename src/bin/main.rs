// Файл, который лежит в src/bin/*.rs образует crate-исполняемый файл (main)
// Из main можно пользоваться только тем, что выставлено наружу (pub) из библиотечного crate и корректно объявлено в lib.rs
use backend::lib::handlers::handler_return_all_containers;
use backend::lib::http_server::Server;
use backend::lib::req_res_structs::{BodyType, Response};
use backend::lib::request::Request;

fn main() {
    let mut server = Server::new("127.0.0.1:8080").unwrap();

    // регистрация пары path и handlers в Hash-table
    server.GET("/container/", handler_return_all_containers); // 2ой аргумент это тип HandlerFn

    server.POST("/container/:id/reboot", |r: &Request| Response {
        response_code: 200,
        headers: None,
        body: Some(BodyType::Plain(format!(
            "Container ID is: {}\nRebooting...\nTEST",
            r.rest_params.get("id").unwrap()
        ))),
    });

    server.POST("/container/:id/start", |r: &Request| Response {
        response_code: 200,
        headers: None,
        body: Some(BodyType::Plain(format!(
            "Container ID is: {}\nStarting...\nTEST",
            r.rest_params.get("id").unwrap()
        ))),
    });

    server.POST("/container/:id/stop", |r: &Request| Response {
        response_code: 200,
        headers: None,
        body: Some(BodyType::Plain(format!(
            "Container ID is: {}\nStopping...\nTEST",
            r.rest_params.get("id").unwrap()
        ))),
    });

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
│   | "/container/:id/reboot" → to_be_determined  |
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
