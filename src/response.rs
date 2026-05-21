use crate::status_messages;
use std::collections::HashMap;

#[derive(Default)]
pub struct HttpResponse {
    pub status: u16,
    body: String,
    pub headers: HashMap<String, String>,
}

impl HttpResponse {
    pub fn default() -> Self {
        Self {
            status: 200,
            ..Default::default()
        }
    }
    pub fn to_string(&self) -> String {
        let mut response_string = String::new();
        let status_message = status_messages::get_status_message(self.status);

        if status_message.is_empty() {
            response_string.push_str("HTTP/1.1 400 Bad Request\r\n\r\n");
            return response_string;
        }
        response_string.push_str(&format!("HTTP/1.1 {} {}\r\n", self.status, status_message));
        for header in &self.headers {
            response_string.push_str(&format!("{}: {}\r\n", header.0, header.1));
        }
        response_string.push_str("\r\n");
        response_string.push_str(&self.body);
        response_string
    }
    pub fn send_json(&mut self, json: serde_json::Value) {
        self.body = json.to_string();
        self.headers
            .insert("Content-Length".to_string(), self.body.len().to_string());
        self.headers
            .insert("Content-Type".to_string(), "application/json".to_owned());
    }
    pub fn send_html(&mut self, html: impl Into<String>) {
        self.body = html.into();
        self.headers
            .insert("Content-Length".to_string(), self.body.len().to_string());
        self.headers
            .insert("Content-Type".to_string(), "text/html".to_owned());
    }
}
