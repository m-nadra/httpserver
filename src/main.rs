use chrono::Local;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

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
    let mut request = Vec::new();
    loop {
        let bytes_read = socket.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            return;
        }
        request.extend(&buffer[..bytes_read]);
        if request.windows(4).any(|w| w == b"\r\n\r\n") || request.windows(2).any(|w| w == b"\n\n")
        {
            break;
        }
    }
    parse_request(&request);
}

fn parse_request(request: &Vec<u8>) {
    let request_string = String::from_utf8_lossy(&request);
    println!("### HTTP Request ###\n\n{}", request_string);

    let lines = request_string.split("\n").next().unwrap();
    let mut parts = lines.split_whitespace();

    let method = parts.next().unwrap();
    let path = parts.next().unwrap();
    println!("{method} - {path}")
}
