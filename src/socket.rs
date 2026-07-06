use crate::http_response::HttpResponse;
use std::error::Error;
use std::io::Write;
use std::net::TcpStream;

pub fn write_response(
    socket: &mut TcpStream,
    response: &HttpResponse,
) -> Result<(), Box<dyn Error>> {
    socket.write_all(&response.to_bytes())?;
    Ok(())
}