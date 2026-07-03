use std::collections::HashMap;

pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    fn extract_query_params(path_with_components: &str) -> (String, HashMap<String, String>) {
        let mut query = HashMap::new();
        let path;

        if let Some((query_path, query_components)) = path_with_components.split_once("?") {
            path = query_path.to_string();

            for param in query_components.split("&") {
                if let Some((key, value)) = param.split_once("=") {
                    query.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
        } else {
            path = path_with_components.to_string();
        }
        (path, query)
    }
    pub fn new(request: String) -> Self {
        let mut lines = request.split("\n");
        let request_start_line: Vec<&str> = lines.next().unwrap().split_whitespace().collect();

        let method = request_start_line[0].to_string();
        let (path, query) = HttpRequest::extract_query_params(request_start_line[1]);
        let version = request_start_line[2].to_string();

        // Extract request headers
        let mut headers = HashMap::new();
        for line in lines.by_ref() {
            if line.trim().is_empty() {
                break;
            }
            if let Some((key, value)) = line.split_once(":") {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        let body = lines.collect::<Vec<_>>().join("\n");

        Self {
            method,
            path,
            version,
            headers,
            body,
            query,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::http_request::HttpRequest;
    #[test]
    fn none_query_params() {
        let (path, query) = HttpRequest::extract_query_params(&"/".to_string());
        assert_eq!(path, "/");
        assert_eq!(query.len(), 0);
    }
    #[test]
    fn one_query_params() {
        let (path, query) = HttpRequest::extract_query_params(&"/name?a=14".to_string());
        assert_eq!(path, "/name".to_string());
        assert_eq!(query.len(), 1);
        assert_eq!(query.get("a"), Some(&"14".to_string()));
    }
    #[test]
    fn multiple_query_params() {
        let (path, query) =
            HttpRequest::extract_query_params(&"/name/profile?a=14&b=134".to_string());
        assert_eq!(path, "/name/profile".to_string());
        assert_eq!(query.len(), 2);
        assert_eq!(query.get("a"), Some(&"14".to_string()));
        assert_eq!(query.get("b"), Some(&"134".to_string()));
    }
    #[test]
    fn invalid_path() {
        let (mut path, mut query);

        (path, query) = HttpRequest::extract_query_params(&"/name/profile?".to_string());
        assert_eq!(path, "/name/profile".to_string());
        assert_eq!(query.len(), 0);

        (path, query) = HttpRequest::extract_query_params(&"/name/profile?aaa&b=13".to_string());
        assert_eq!(path, "/name/profile".to_string());
        assert_eq!(query.len(), 1);
        assert!(!query.contains_key("aaa"));
        assert_eq!(query.get("b"), Some(&"13".to_string()));

        (path, query) = HttpRequest::extract_query_params(&"/name/profile?aaa=&bbb=12".to_string());
        assert_eq!(path, "/name/profile".to_string());
        assert_eq!(query.len(), 2);
        assert_eq!(query.get("aaa"), Some(&"".to_string()));
        assert_eq!(query.get("bbb"), Some(&"12".to_string()));
    }
}
