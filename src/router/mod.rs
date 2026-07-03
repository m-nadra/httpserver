mod http_methods;

use crate::{HttpRequest, HttpResponse};
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
}
