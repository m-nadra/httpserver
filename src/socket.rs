use crate::router;
use crate::{HttpRequest, HttpResponse, Router};
use std::net::TcpStream;

use std::io::{Read, Write};

const BUFFER_SIZE: usize = 1024;

pub fn handle_stream(mut socket: TcpStream, router: &Router) {
    let stream = read_from_socket(&mut socket);

    let request = HttpRequest::new(stream);
    let mut response = HttpResponse::default();

    router::route_to_endpoint(request, &mut response, router);

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
