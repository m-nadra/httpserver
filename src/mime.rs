pub fn get_mime_type(file_type: &str) -> String {
    let mime_type = match file_type {
        "html" => "text/html",
        "txt" => "text/plain",
        "css" => "text/css",
        "js" => "text/javascript",
        "xml" => "text/xml",
        "pdf" => "application/pdf",
        _ => "application/octet-stream",
    };
    mime_type.to_string()
}
