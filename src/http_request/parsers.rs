use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

type StartLine = (String, String, HashMap<String, String>, String);

pub fn parse_start_line(stream: &mut BufReader<&TcpStream>) -> Result<StartLine, Box<dyn Error>> {
    let mut request_start_line = String::new();
    stream.read_line(&mut request_start_line)?;

    let request_line: Vec<&str> = request_start_line.split_whitespace().collect();
    let (path, query) = extract_query_params(request_line[1]);
    Ok((
        request_line[0].to_string(),
        path,
        query,
        request_line[2].to_string(),
    ))
}
pub fn parse_headers(
    stream: &mut BufReader<&TcpStream>,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut headers = HashMap::new();
    loop {
        let mut header = String::new();
        if stream.read_line(&mut header)? == 0 {
            break;
        }

        if header.trim_end().is_empty() {
            break;
        }

        if let Some((key, value)) = header.split_once(":") {
            headers.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    Ok(headers)
}

pub fn parse_body(
    stream: &mut BufReader<&TcpStream>,
    body_length: usize,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut body = vec![0u8; body_length];
    stream.read_exact(&mut body)?;
    Ok(body)
}
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn none_query_params() {
        let (path, query) = extract_query_params(&"/".to_string());
        assert_eq!(path, "/");
        assert_eq!(query.len(), 0);
    }
    #[test]
    fn one_query_params() {
        let (path, query) = extract_query_params(&"/name?a=14".to_string());
        assert_eq!(path, "/name".to_string());
        assert_eq!(query.len(), 1);
        assert_eq!(query.get("a"), Some(&"14".to_string()));
    }
    #[test]
    fn multiple_query_params() {
        let (path, query) = extract_query_params(&"/name/profile?a=14&b=134".to_string());
        assert_eq!(path, "/name/profile".to_string());
        assert_eq!(query.len(), 2);
        assert_eq!(query.get("a"), Some(&"14".to_string()));
        assert_eq!(query.get("b"), Some(&"134".to_string()));
    }
    #[test]
    fn invalid_path() {
        let (mut path, mut query);

        (path, query) = extract_query_params(&"/name/profile?".to_string());
        assert_eq!(path, "/name/profile".to_string());
        assert_eq!(query.len(), 0);

        (path, query) = extract_query_params(&"/name/profile?aaa&b=13".to_string());
        assert_eq!(path, "/name/profile".to_string());
        assert_eq!(query.len(), 1);
        assert!(!query.contains_key("aaa"));
        assert_eq!(query.get("b"), Some(&"13".to_string()));

        (path, query) = extract_query_params(&"/name/profile?aaa=&bbb=12".to_string());
        assert_eq!(path, "/name/profile".to_string());
        assert_eq!(query.len(), 2);
        assert_eq!(query.get("aaa"), Some(&"".to_string()));
        assert_eq!(query.get("bbb"), Some(&"12".to_string()));
    }
}
