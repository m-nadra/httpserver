use super::*;

impl Router {
    pub fn get(&mut self, path: impl Into<String>, function: FunctionHandler) {
        self.routes.push(Route {
            method: "GET".to_owned(),
            path: path.into(),
            function,
        });
    }
    pub fn post(&mut self, path: impl Into<String>, function: FunctionHandler) {
        self.routes.push(Route {
            method: "POST".to_owned(),
            path: path.into(),
            function,
        });
    }
    pub fn put(&mut self, path: impl Into<String>, function: FunctionHandler) {
        self.routes.push(Route {
            method: "PUT".to_owned(),
            path: path.into(),
            function,
        });
    }
    pub fn delete(&mut self, path: impl Into<String>, function: FunctionHandler) {
        self.routes.push(Route {
            method: "DELETE".to_owned(),
            path: path.into(),
            function,
        });
    }
    pub fn patch(&mut self, path: impl Into<String>, function: FunctionHandler) {
        self.routes.push(Route {
            method: "PATCH".to_owned(),
            path: path.into(),
            function,
        });
    }
}
