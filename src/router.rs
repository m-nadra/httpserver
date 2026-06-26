use crate::response::Disposition;
use crate::socket;
use crate::{HttpRequest, HttpResponse};
use std::collections::HashMap;
use std::net::TcpListener;
type FunctionHandler = fn(&HttpRequest, &mut HttpResponse);

struct Route {
    path: String,
    method: String,
    function: FunctionHandler,
}

#[derive(Default)]
pub struct Router {
    routes: Vec<Route>,
    statics: HashMap<String, String>,
}

impl Router {
    pub fn mount_static(&mut self, path: impl Into<String>, directory: impl Into<String>) {
        self.statics.insert(path.into(), directory.into());
    }
    pub fn get(&mut self, path: impl Into<String>, function: FunctionHandler) {
        self.routes.push(Route {
            method: "GET".to_owned(),
            path: path.into(),
            function,
        });
    }
    pub fn listen(&self, port: u16) {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            socket::handle_stream(stream, self);
        }
    }
}

pub fn route_to_endpoint(request: &HttpRequest, response: &mut HttpResponse, router: &Router) {
    let path = &request.path;
    let method = &request.method;

    // Static content serving
    for (route, dir) in router.statics.iter() {
        if path.starts_with(route) {
            let mut file_path = dir.clone();
            file_path.push_str(&path.clone().split_off(route.len()));
            if response.send_file(file_path, Disposition::Inline).is_err() {
                response.status = 404;
            }
            return;
        }
    }

    // Endpoint matching
    let mut status = 404;

    for route in &router.routes {
        if route.path == *path {
            status = 405;
            if route.method == *method {
                status = 200;
                (route.function)(request, response);
                break;
            }
        }
    }
    response.status = status;
}
