// Файл, который лежит в src/bin/*.rs образует crate-исполняемый файл (main)
// Из main можно пользоваться только тем, что выставлено наружу (pub) из библиотечного crate и корректно объявлено в lib.rs
use backend::lib::req_res_structs::{BodyType, Response};
use backend::lib::{http_server::Server, parse_funcs::deser_response};
fn main() {
    let server = Server::new("127.0.0.1:8080").unwrap();

    server.GET("/home".to_string(), handler_home)?;

    server.start();
}
