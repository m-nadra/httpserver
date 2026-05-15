use chrono::Local;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

mod request;
use request::HttpRequest;

const PORT: u16 = 3000;
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

    let mut buffer = [0; 1024];
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
    println!("{}", request.body)
}
