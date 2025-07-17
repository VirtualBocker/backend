// Файл, который лежит в src/bin/*.rs образует crate-исполняемый файл (main)
// Из main можно пользоваться только тем, что выставлено наружу (pub) из библиотечного crate и корректно объявлено в lib.rs
use backend::lib::req_res_structs::{BodyType, Response};
use backend::lib::request::Request;
use backend::lib::{http_server::Server, parse_funcs::deser_response};
fn main() {
    let mut server = Server::new("127.0.0.1:8080").unwrap();

    // server.POST("/container/:id/delete", |r: &Request| {
    //     Response { response_code: 200, headers: None, body: None }
    // });

    server.start();
}
