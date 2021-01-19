use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SocketConnectionError {
    reason: String,
    path: String,
}

impl SocketConnectionError {
    pub fn from(path: &str, e: &dyn Error) -> Self {
        SocketConnectionError {
            reason: format!("{}", e),
            path: path.to_string(),
        }
    }
}

impl Error for SocketConnectionError {}
impl fmt::Display for SocketConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unable to connect socket: {} ({})",
            self.path, self.reason
        )
    }
}
