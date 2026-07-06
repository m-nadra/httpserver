use std::collections::HashMap;
use std::error::Error;
use std::io::BufReader;
use std::net::TcpStream;

mod parsers;

pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl HttpRequest {
    pub fn new(request_stream: &TcpStream) -> Result<Self, Box<dyn Error>> {
        let mut buffer = BufReader::new(request_stream);

        let (method, path, query, version) = parsers::parse_start_line(&mut buffer)?;
        let headers = parsers::parse_headers(&mut buffer)?;
        let body = match headers.get("Content-Length") {
            Some(body_length) => Some(parsers::parse_body(
                &mut buffer,
                body_length.parse::<usize>()?,
            )?),
            None => None,
        };
        Ok(Self {
            method,
            path,
            version,
            headers,
            body,
            query,
        })
    }
}
