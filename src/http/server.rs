use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

pub struct Server {
    port: i16
}

impl Server {
    pub fn new(port: i16) -> Self {
        return Self { port };
    }

    fn handle_request(&self, stream: &mut TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        self.log_request(&buffer[..]);

        self.send_json_response(stream);
    }

    fn send_response(&self, stream: &mut TcpStream) {
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn send_json_response(&self, stream: &mut TcpStream) {
        let data = "{\"name\":\"some_name\",\"value\":\"some_value\"}";
        let response = format!("HTTP/1.1 200 OK\r\nContent-length:{}\r\nContent-Type:application/json\r\n\r\n{}",
                               data.len(), data);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn send_html_response(&self, stream: &mut TcpStream) {
        let content = fs::read_to_string("index.html").unwrap();
        let res = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                          content.len(), content);
        stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn log_request(&self, buffer: &[u8]) {
        println!("Request: {}", String::from_utf8_lossy(&buffer));
    }

    pub fn start(&self) {
        let bindingPath = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(bindingPath).unwrap();
        println!("Server is running on port {}", self.port);
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => self.handle_request(&mut stream),
                Err(e) => println!("couldn't get client: {}", e),
            }
        }
    }
}
