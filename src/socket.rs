use crate::{HttpRequest, HttpResponse, Router};
use chrono::Local;
use std::fs::File;
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

    if let Some(path) = router.get_static_content_path(&request.path) {
        response
            .send_file(path, crate::Disposition::Inline)
            .unwrap_or_else(|_| response.status = 404);
    } else {
        match router.get_function_handler(&request.path, &request.method) {
            Ok(func) => func(&request, &mut response),
            Err(code) => response.status = code,
        }
    }

    // Combined Log Format
    let access_log = format!(
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

    println!("{access_log}");

    const ACCESS_LOG_FILE_PATH: &str = "access_log.txt";
    if let Ok(mut access_log_file) = File::options()
        .append(true)
        .create(true)
        .open(ACCESS_LOG_FILE_PATH)
    {
        access_log_file.write_all(&access_log.into_bytes()).unwrap();
        access_log_file.write_all("\n".as_bytes()).unwrap();
    } else {
        println!("Can't write log to {ACCESS_LOG_FILE_PATH}")
    }
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
