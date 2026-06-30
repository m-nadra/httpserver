use std::fs::File;
use std::io::Write;

pub struct Logger {
    access_logfile: String,
}

impl Logger {
    pub fn default(access_logfile: String) -> Self {
        Self {
            access_logfile,
        }
    }
    pub fn log(&self, message: String) {
        if let Ok(mut access_log_file) = File::options()
            .append(true)
            .create(true)
            .open(&self.access_logfile)
        {
            access_log_file.write_all(message.as_bytes()).unwrap();
            access_log_file.write_all("\n".as_bytes()).unwrap();
        } else {
            println!("Can't write log to {}", &self.access_logfile)
        }
    }
}
