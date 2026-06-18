use crate::mime::get_mime_type;
use crate::status_messages;
use std::collections::HashMap;
use std::fs;

#[derive(Default)]
pub struct HttpResponse {
    pub status: u16,
    body: Vec<u8>,
    pub headers: HashMap<String, String>,
}

impl HttpResponse {
    pub fn default() -> Self {
        Self {
            status: 200,
            ..Default::default()
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response_bytes = Vec::new();
        let status_message = status_messages::get_status_message(self.status);

        if status_message.is_empty() {
            response_bytes.extend_from_slice(b"HTTP/1.1 400 Bad Request\r\n\r\n");
            return response_bytes;
        }
        response_bytes.extend_from_slice(
            format!("HTTP/1.1 {} {}\r\n", self.status, status_message).as_bytes(),
        );
        for header in &self.headers {
            response_bytes.extend_from_slice(format!("{}: {}\r\n", header.0, header.1).as_bytes());
        }
        response_bytes.extend_from_slice(b"\r\n");
        response_bytes.extend_from_slice(&self.body);
        response_bytes
    }
    pub fn send_text(&mut self, body: impl Into<String>) {
        self.body = body.into().into_bytes();
        self.headers
            .insert("Content-Length".to_string(), self.body.len().to_string());
        self.headers
            .insert("Content-Type".to_string(), "text/plain".to_owned());
    }
    pub fn send_json(&mut self, json: serde_json::Value) {
        self.body = json.to_string().into_bytes();
        self.headers
            .insert("Content-Length".to_string(), self.body.len().to_string());
        self.headers
            .insert("Content-Type".to_string(), "application/json".to_owned());
    }
    pub fn send_html(&mut self, path: impl Into<String>) {
        self.body = fs::read(path.into()).expect("Can't access file");
        self.headers
            .insert("Content-Length".to_string(), self.body.len().to_string());
        self.headers
            .insert("Content-Type".to_string(), "text/html".to_owned());
    }
    pub fn send_file(&mut self, path: impl Into<String>) {
        let path = path.into();
        self.body = fs::read(&path).expect("Can't access file");
        self.headers
            .insert("Content-Length".to_string(), self.body.len().to_string());

        let file_extesion = path.split(".").last().unwrap();
        self.headers
            .insert("Content-Type".to_string(), get_mime_type(file_extesion));
    }
    pub fn send_attachment(&mut self, path: impl Into<String>) {
        self.body = fs::read(path.into()).expect("Can't access file");
        self.headers
            .insert("Content-Length".to_string(), self.body.len().to_string());
        self.headers
            .insert("Content-Disposition".to_string(), "attachment".to_owned());
    }
}
