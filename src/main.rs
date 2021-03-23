use http::Server;

mod http;

fn main() {
    let server = Server::new(8080);
    server.start();
}
