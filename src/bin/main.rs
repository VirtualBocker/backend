// Файл, который лежит в src/bin/*.rs образует crate-исполняемый файл (main)
// Из main можно пользоваться только тем, что выставлено наружу (pub) из библиотечного crate и корректно объявлено в lib.rs
use backend::lib::docker_works::{
    ContainerError, ContainerInfo, parse_docker_ps_a, start_container, stop_container,
};
use backend::lib::http_server::Server;
use backend::lib::parse_funcs::parse_request;
fn main() {
    let server = Server::new("127.0.0.1:8080").unwrap();
    server.start();
}
