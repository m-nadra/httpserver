use super::{Disposition, HttpResponse, mime::get_mime_type};
use std::error::Error;
use std::fs;

impl HttpResponse {
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
    pub fn send_html(&mut self, content: impl Into<Vec<u8>>) {
        self.body = content.into();
        self.headers
            .insert("Content-Length".to_string(), self.body.len().to_string());
        self.headers
            .insert("Content-Type".to_string(), "text/html".to_owned());
    }
    pub fn send_file(
        &mut self,
        path: impl Into<String>,
        render_type: Disposition,
    ) -> Result<(), Box<dyn Error>> {
        let path = path.into();
        self.body = fs::read(&path)?;
        self.headers
            .insert("Content-Length".to_string(), self.body.len().to_string());

        let file_extesion = path.split(".").last().unwrap();
        self.headers
            .insert("Content-Type".to_string(), get_mime_type(file_extesion));

        self.headers
            .insert("Content-Disposition".to_string(), render_type.value());

        Ok(())
    }
}
