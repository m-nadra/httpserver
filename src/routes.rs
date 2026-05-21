use crate::{HttpRequest, HttpResponse};

type FunctionHandler = fn(&HttpRequest, &mut HttpResponse);

struct Route {
    path: String,
    method: String,
    function: FunctionHandler,
}

struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn default() -> Self {
        Self { routes: Vec::new() }
    }
    pub fn get(&mut self, path: String, function: FunctionHandler) {
        self.routes.push(Route {
            method: "GET".to_owned(),
            path,
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
}

pub fn router(request: &HttpRequest, response: &mut HttpResponse) {
    let mut router = Router::default();
    router.get("/".to_owned(), |_, res| {
        res.send_html("<h1>Default route</h1>".to_owned());
    });
    router.get("/json".to_owned(), |_, res| {
        res.send_json("{\"message\": \"Hello World!\"}".to_owned());
    });

    match router.find_endpoint(&request.path, &request.method) {
        Ok(func) => func(request, response),
        Err(code) => response.status = code,
    };
}
