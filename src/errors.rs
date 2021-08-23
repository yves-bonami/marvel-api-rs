use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ApiError {
    HttpError(attohttpc::Error),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub type Result<T, E = ApiError> = std::result::Result<T, E>;
