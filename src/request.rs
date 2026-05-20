use std::collections::HashMap;

pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    pub fn new(request: String) -> Self {
        let mut lines = request.split("\n");
        let mut parts = lines.next().unwrap().split_whitespace();
        let mut headers = HashMap::new();
        for line in lines.by_ref() {
            if line.trim().is_empty() {
                break;
            }
            let (key, value) = line.split_once(":").unwrap();
            headers.insert(key.trim().to_string(), value.trim().to_string());
        }
        let body = lines.collect::<Vec<_>>().join("\n");
        Self {
            method: parts.next().unwrap().to_string(),
            path: parts.next().unwrap().to_string(),
            headers,
            body,
        }
    }
}
