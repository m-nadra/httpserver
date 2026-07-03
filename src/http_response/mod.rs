use std::collections::HashMap;

mod mime;
mod senders;
mod status_messages;

pub enum Disposition {
    Inline,
    Attachment,
}

impl Disposition {
    fn value(&self) -> String {
        let value = match *self {
            Disposition::Inline => "inline",
            Disposition::Attachment => "attachment",
        };
        value.to_string()
    }
}

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
}
