use backend::lib::http_server::Server;

fn main() {
    let server = Server::new("127.0.0.1:8080").unwrap();
    server.start();
}
