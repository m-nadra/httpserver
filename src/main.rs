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
    let bytes_read = socket.read(&mut buffer).unwrap();
    println!("Read {bytes_read} bytes!");

    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("received: {}", message);

    if message.trim() == "PING" {
        socket.write_all(b"PONG").unwrap();
    } else {
        socket.write_all(format!("{message}").as_bytes()).unwrap();
    }
}