use core::fmt;

#[derive(Debug)]
pub enum ApiError {
    Fault(Fault),
    Transport(isahc::Error),
    Error(std::io::Error),
}

#[derive(Debug, Deserialize)]
pub struct Fault {
    code: String,
    message: String,
}

#[derive(Debug, Deserialize)]
pub struct Error {
    code: String,
    field: String,
    resource: String,
}

impl std::error::Error for Fault {}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(f, "Error: {}", &self.message)
    }
}

impl From<Fault> for ApiError {
    fn from(err: Fault) -> Self {
        ApiError::Fault(err)
    }
}

impl From<isahc::Error> for ApiError {
    fn from(err: isahc::Error) -> Self {
        ApiError::Transport(err)
    }
}
