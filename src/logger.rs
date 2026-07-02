use std::{
    error::Error,
    fs::{self, File},
    io::{Write, stderr, stdout},
    path::Path,
};

/// Writes access and error logs.
///
/// By default, access logs are written to stdout and error logs to stderr.
/// Each output can be redirected independently to a file.
pub struct Logger {
    access_writer: Box<dyn Write>,
    error_writer: Box<dyn Write>,
}

impl Logger {
    /// Creates a new logger.
    ///
    /// Access logs are written to stdout and error logs to stderr.
    pub fn new() -> Self {
        Self {
            access_writer: Box::new(stdout()),
            error_writer: Box::new(stderr()),
        }
    }

    /// Opens a log file for appending.
    ///
    /// Missing parent directories are created automatically.
    fn open_log_file(&self, path: &Path) -> Result<File, Box<dyn Error>> {
        fs::create_dir_all(path.parent().unwrap_or(Path::new("")))?;
        Ok(File::options().append(true).create(true).open(path)?)
    }

    /// Redirects access logs to a file.
    ///
    /// Missing parent directories are created automatically.
    ///
    /// If the file cannot be opened, the current output destination is left
    /// unchanged and an error message is printed to stderr.
    pub fn set_access_file<T: AsRef<Path>>(&mut self, path: T) {
        let path = path.as_ref();
        match self.open_log_file(path) {
            Ok(fd) => {
                println!("[LOGGER] Access logs are saved to {}", path.display());
                self.access_writer = Box::new(fd);
            }
            Err(e) => eprintln!("[LOGGER] Can't open access log file: {e}"),
        }
    }

    /// Redirects log output to the stdout.
    pub fn set_access_stdout(&mut self) {
        self.access_writer = Box::new(stdout());
    }

    /// Redirects error logs to a file.
    ///
    /// Missing parent directories are created automatically.
    ///
    /// If the file cannot be opened, the current output destination is left
    /// unchanged and an error message is printed to stderr.
    pub fn set_error_file<T: AsRef<Path>>(&mut self, path: T) {
        let path = path.as_ref();
        match self.open_log_file(path) {
            Ok(fd) => {
                println!("[LOGGER] Error logs are saved to {}", path.display());
                self.error_writer = Box::new(fd);
            }
            Err(e) => eprintln!("[LOGGER] Can't open error log file: {e}"),
        }
    }

    /// Redirects log output to the stderr.
    pub fn set_error_stderr(&mut self) {
        self.error_writer = Box::new(stderr());
    }

    /// Writes an access log entry.
    ///
    /// A trailing newline is appended automatically.
    ///
    /// If writing fails, the error is reported to stderr.
    pub fn log_access<T: AsRef<str>>(&mut self, message: T) {
        writeln!(self.access_writer, "{}", message.as_ref())
            .unwrap_or_else(|err| eprintln!("Failed to write access log: {err}"));
    }

    /// Writes an error log entry.
    ///
    /// A trailing newline is appended automatically.
    ///
    /// If writing fails, the error is reported to stderr.
    pub fn log_error<T: AsRef<str>>(&mut self, message: T) {
        writeln!(self.error_writer, "{}", message.as_ref())
            .unwrap_or_else(|err| eprintln!("Failed to write error log: {err}"));
    }
}
