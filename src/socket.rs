use crate::router;
use crate::{HttpRequest, HttpResponse, Router};
use chrono::Local;
use std::net::TcpStream;

use std::io::{Read, Write};

const BUFFER_SIZE: usize = 1024;

pub fn handle_stream(mut socket: TcpStream, router: &Router) {
    let stream = read_from_socket(&mut socket);
    let request_data = {
        let end = stream.find("\r\n").unwrap_or(stream.len());
        String::from(&stream[..end])
    };

    let request = HttpRequest::new(stream);
    let mut response = HttpResponse::default();

    router::route_to_endpoint(&request, &mut response, router);

    // Combined Log Format
    println!(
        "{} - - [{}] \"{}\" {} {} {} {}", // TODO! Add authorized user instead second hyphen
        socket.peer_addr().unwrap().ip(),
        Local::now().format("%d/%b/%Y:%H:%M:%S %z"),
        request_data,
        response.status,
        response
            .headers
            .get("Content-Length")
            .unwrap_or(&"0".to_string()),
        request
            .headers
            .get("Referer")
            .unwrap_or(&"-".to_string())
            .clone(),
        request
            .headers
            .get("User-Agent")
            .unwrap_or(&"-".to_string())
            .clone()
    );

    socket.write_all(&response.to_bytes()).unwrap();
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
