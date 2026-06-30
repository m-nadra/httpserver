use crate::{HttpRequest, HttpResponse};
use std::collections::HashMap;
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
    pub fn get_function_handler(
        &self,
        path: &String,
        method: &String,
    ) -> Result<FunctionHandler, u16> {
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
    pub fn get_static_content_path(&self, path: &str) -> Option<String> {
        for (route, dir) in self.statics.iter() {
            if path.starts_with(route) {
                let mut file_path = dir.clone();
                file_path.push_str(&path.to_owned().split_off(route.len()));
                return Some(file_path);
            }
        }
        None
    }
}
