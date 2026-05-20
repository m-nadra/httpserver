use chrono::Local;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

mod request;
use request::HttpRequest;

mod response;
use response::HttpResponse;

mod routes;
mod status_messages;

const PORT: u16 = 3000;
const BUFFER_SIZE: usize = 1024;

fn main() {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let now = Local::now().format("%Y-%m-%d %H:%M:%S");
        let addr = stream.peer_addr().unwrap();
        println!("{now} => {addr:?}");
        handle_stream(stream);
    }
}

fn handle_stream(mut socket: TcpStream) {
    let stream = read_from_socket(&mut socket);

    let request = HttpRequest::new(stream);
    let mut response = HttpResponse::new(200);

    routes::router(&request, &mut response);

    socket.write_all(response.to_string().as_bytes()).unwrap();
}

fn read_from_socket(socket: &mut TcpStream) -> String {
    let mut buffer = [0; BUFFER_SIZE];
    let mut stream = Vec::new();
    loop {
        let bytes_read = socket.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        stream.extend(&buffer[..bytes_read]);
        if stream.windows(4).any(|w| w == b"\r\n\r\n") || stream.windows(2).any(|w| w == b"\n\n") {
            break;
        }
    }
    String::from_utf8(stream).unwrap()
}
