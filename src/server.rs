use crate::Disposition;
use crate::Logger;
use crate::Router;
use crate::Static;
use crate::socket;
use crate::{HttpRequest, HttpResponse};
use chrono::Local;
use std::error::Error;
use std::net::{TcpListener, TcpStream};

pub struct Server {
    routers: Vec<Router>,
    pub logger: Logger,
    statics: Static,
}

impl Server {
    pub fn create() -> Self {
        Self {
            routers: Vec::new(),
            logger: Logger::new(),
            statics: Static::default(),
        }
    }

    pub fn mount(&mut self, router: Router) {
        self.routers.push(router);
    }

    pub fn mount_static<T: Into<String>>(&mut self, path: T, directory: T) {
        self.statics.insert(path.into(), directory.into());
    }

    pub fn listen<T: Into<String>>(&mut self, socket: T) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(socket.into())?;

        for stream in listener.incoming() {
            let stream = stream?;
            // Add thread spawning
            let message = self.handle_stream(stream);
            self.logger.log_access(message);
        }
        Ok(())
    }

    fn handle_stream(&self, mut socket: TcpStream) -> String {
        let stream = socket::read_request(&mut socket).unwrap();

        let request = HttpRequest::new(stream);
        let mut response = HttpResponse::default();
        

        // Rewrite routing
        if let Some(path) = self.statics.get_content_path(&request.path) {
            response
                .send_file(path, Disposition::Inline)
                .unwrap_or_else(|_| response.status = 404);
        } else {
            for router in &self.routers {
                match router.get_function_handler(&request.path, &request.method) {
                    Ok(func) => {
                        response.status = 200;
                        func(&request, &mut response);
                        break;
                    }
                    Err(code) => response.status = if response.status == 405 { 405 } else { code },
                }
            }
        }

        socket::write_response(&mut socket, &response).unwrap();
        format!(
            "{} - - [{}] \"{} {} {}\" {} {} {} {}", // TODO! Add authorized user instead second hyphen
            socket.peer_addr().unwrap().ip(),
            Local::now().format("%d/%b/%Y:%H:%M:%S %z"),
            request.method,
            request.path,
            request.version,
            response.status,
            response
                .headers
                .get("Content-Length")
                .unwrap_or(&"0".to_string()),
            request
                .headers
                .get("Referer")
                .unwrap_or(&"-".to_string())
                .clone(),
            request
                .headers
                .get("User-Agent")
                .unwrap_or(&"-".to_string())
                .clone()
        )
    }
}
