use crate::response::HttpResponse;
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

const BUFFER_SIZE: usize = 1024;

pub fn read_request(socket: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    let mut buffer = [0; BUFFER_SIZE];
    let mut stream = Vec::new();
    loop {
        let bytes_read = socket.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        stream.extend(&buffer[..bytes_read]);
        if stream.windows(4).any(|w| w == b"\r\n\r\n") || stream.windows(2).any(|w| w == b"\n\n") {
            break;
        }
    }
    Ok(String::from_utf8(stream)?)
}

pub fn write_response(
    socket: &mut TcpStream,
    response: &HttpResponse,
) -> Result<(), Box<dyn Error>> {
    socket.write_all(&response.to_bytes())?;
    Ok(())
}
