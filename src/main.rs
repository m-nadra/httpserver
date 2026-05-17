use chrono::Local;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

mod request;
use request::HttpRequest;

mod response;
use response::HttpResponse;

mod status_messages;

const PORT: u16 = 3000;
const BUFFER_SIZE: usize = 1024;

fn main() {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT)).unwrap();

    for stream in listener.incoming() {
        handle_stream(stream.unwrap());
    }
}

fn handle_stream(mut socket: TcpStream) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    let addr = socket.peer_addr().unwrap();
    println!("{now} => {addr:?}");

    let mut buffer = [0; BUFFER_SIZE];
    let mut stream = Vec::new();
    loop {
        let bytes_read = socket.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            return;
        }
        stream.extend(&buffer[..bytes_read]);
        if stream.windows(4).any(|w| w == b"\r\n\r\n") || stream.windows(2).any(|w| w == b"\n\n") {
            break;
        }
    }
    let request = HttpRequest::new(&stream);
    println!("{} - {}", request.method, request.path);
    println!("{:?}", request.headers);
    println!("{}", request.body);

    let response = HttpResponse::new(200);
    socket.write_all(response.to_string().as_bytes()).unwrap();
}
