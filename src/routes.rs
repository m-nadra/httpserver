use std::collections::HashMap;

use crate::{HttpRequest, HttpResponse};

fn default(request: &HttpRequest, response: &mut HttpResponse) {
    println!("{} - {}", request.method, request.path);
    println!("{:?}", request.headers);
    println!("{}", request.body);
    response.body = "<h1>Hello World</h1>".to_owned();
}

fn json(request: &HttpRequest, response: &mut HttpResponse) {
    println!("{} - {}", request.method, request.path);
    println!("{:?}", request.headers);
    println!("{}", request.body);
    response.body = "{\"message\": \"Hello World!\"}".to_owned();
}
pub fn router(request: &HttpRequest, response: &mut HttpResponse) {
    let mut routes: HashMap<String, fn(&HttpRequest, &mut HttpResponse)> = HashMap::new();
    routes.insert("/".to_owned(), default);
    routes.insert("/json".to_owned(), json);

    if let Some(func) = routes.get(&request.path) {
        func(request, response)
    }
}
