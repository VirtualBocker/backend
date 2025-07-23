use backend::lib::config::Config;
// Файл, который лежит в src/bin/*.rs образует crate-исполняемый файл (main)
// Из main можно пользоваться только тем, что выставлено наружу (pub) из библиотечного crate и корректно объявлено в lib.rs
use backend::lib::handlers::{
    handler_restart_container, handler_return_all_containers, handler_start_container,
    handler_stop_container,
};
use backend::lib::http_server::Server;
use backend::lib::req_res_structs::{BodyType, Response};
use backend::lib::request::Request;

fn main() {
    let conf = Config::from_env().with_port(8080);

    let mut server = Server::with_config(conf).unwrap();

    server
        .log
        .debug(&format!("ip_port = {}", server.config.port));

    // регистрация пары path и handlers в Hash-table
    server.GET("/container/", handler_return_all_containers); // 2ой аргумент это тип HandlerFn

    server.POST("/container/:id/restart", handler_restart_container);
    server.POST("/container/:id/start", handler_start_container);
    server.POST("/container/:id/stop", handler_stop_container);

    // будущие handlers:
    // GET: handler_inspect_container
    // POST: handler_pause_container, handler_unpause_container
    // DELETE: handler_remove_container

    // handler_inspect_container
    server.GET("/container/:id/", |r: &Request| Response {
        response_code: 200,
        headers: None,
        body: Some(BodyType::Plain(format!(
            "Container ID is: {}\nRebooting...\nTEST",
            r.rest_params.get("id").unwrap()
        ))),
    });

    // handler_pause_container
    server.POST("/container/:id/pause", |r: &Request| Response {
        response_code: 200,
        headers: None,
        body: Some(BodyType::Plain(format!(
            "Container ID is: {}\nRebooting...\nTEST",
            r.rest_params.get("id").unwrap()
        ))),
    });

    // handler_unpause_container
    server.POST("/container/:id/unpause", |r: &Request| Response {
        response_code: 200,
        headers: None,
        body: Some(BodyType::Plain(format!(
            "Container ID is: {}\nRebooting...\nTEST",
            r.rest_params.get("id").unwrap()
        ))),
    });

    // handler_remove_container
    server.DELETE("/container/:id/", |r: &Request| Response {
        response_code: 200,
        headers: None,
        body: Some(BodyType::Plain(format!(
            "Container ID is: {}\nRebooting...\nTEST",
            r.rest_params.get("id").unwrap()
        ))),
    });

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

    server.start().unwrap();
}

/* Как сейчас выглядит Hash-table
handlers                          // HashMap<Method, …>
│
├── Method::GET ──┐   // 1‑й уровень
│                 │
│   +------------------------------------------------+  // 2‑й уровень (HashMap<String, HandlerFn>)
│   |  "/container/" → handler_return_all_containers |
│   +------------------------------------------------+
│
├── Method::POST ─┐   // 1‑й уровень
│                 │
│   +------------------------------------------------------+ // 2‑й уровень (HashMap<String, HandlerFn>)
│   | "/container/:id/restart" → handler_restart_container |
│   | "/container/:id/start"   → handler_start_container   |
│   | "/container/:id/stop"    → handler_stop_container    |
│   +------------------------------------------------------+
│
├── Method::PUT  ─┐   // 1‑й уровень
│                 │
│   +---------------------------+ // 2‑й уровень (HashMap<String, HandlerFn>)
│   | EMPTY                     |
│   +---------------------------+
│
└── Method::DELETE ─┐ // 1‑й уровень
                    │
    +---------------------------+ // 2‑й уровень (HashMap<String, HandlerFn>)
    | EMPTY                     |
    +---------------------------+

*/
