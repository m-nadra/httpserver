use std::collections::HashMap;

use crate::{HttpRequest, HttpResponse};

fn default(_: &HttpRequest, response: &mut HttpResponse) {
    response.send_html("<h1>Default route</h1>".to_owned());
}

fn json(_: &HttpRequest, response: &mut HttpResponse) {
    response.send_json("{\"message\": \"Hello World!\"}".to_owned());
}
pub fn router(request: &HttpRequest, response: &mut HttpResponse) {
    let mut routes: HashMap<String, fn(&HttpRequest, &mut HttpResponse)> = HashMap::new();
    routes.insert("/".to_owned(), default);
    routes.insert("/json".to_owned(), json);

    if let Some(func) = routes.get(&request.path) {
        func(request, response);
    } else {
        response.status = 404;
    }
}
