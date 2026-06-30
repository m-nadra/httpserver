use crate::Logger;
use crate::Router;
use crate::socket;
use std::error::Error;
use std::net::TcpListener;

pub struct Server {
    router: Router,
    logger: Logger,
}

impl Server {
    pub fn create() -> Self {
        Self {
            router: Router::default(),
            logger: Logger::default("access_log.txt".to_string()),
        }
    }
    pub fn mount(&mut self, router: Router) {
        self.router = router;
    }
    pub fn listen(&self, socket: impl Into<String>) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(socket.into())?;

        for stream in listener.incoming() {
            let stream = stream?;
            let message = socket::handle_stream(stream, &self.router);

            println!("{message}");
            self.logger.log(message);
        }
        Ok(())
    }
}
