use crate::status_messages;
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

impl HttpResponse {
    pub fn new(status: u16) -> Self {
        Self {
            status,
            body: "<html>Hello world!</html>".to_owned(),
        }
    }

    pub fn to_string(self) -> String {
        let mut response_string = String::new();
        let status_message = status_messages::get_status_message(self.status);

        if status_message.is_empty() {
            response_string.push_str("HTTP/1.1 400 Bad Request\r\n\r\n");
            return response_string;
        }
        response_string.push_str(&format!("HTTP/1.1 {} {}\r\n", self.status, status_message));
        response_string.push_str(&format!("Content-Length: {}\r\n", self.body.len() + 4));
        response_string.push_str("\r\n");
        response_string.push_str(&self.body);
        response_string.push_str("\r\n\r\n");
        response_string
    }
}
