use crate::socket;
use crate::{HttpRequest, HttpResponse};
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
}

impl Router {
    pub fn get(&mut self, path: impl Into<String>, function: FunctionHandler) {
        self.routes.push(Route {
            method: "GET".to_owned(),
            path: path.into(),
            function,
        });
    }
    pub fn find_endpoint(&self, path: &String, method: &String) -> Result<FunctionHandler, u16> {
        let mut status = 404;

        for route in &self.routes {
            if route.path == *path {
                status = 405;
                if route.method == *method {
                    return Ok(route.function);
                }
            }
        }
        Err(status)
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
    match router.find_endpoint(&request.path, &request.method) {
        Ok(func) => func(request, response),
        Err(code) => response.status = code,
    };
}
