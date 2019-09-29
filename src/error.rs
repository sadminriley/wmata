use serde::{Deserialize, Serialize};
use std::{error, fmt};

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Error { message }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorResponse<'a> {
    pub message: &'a str,
}
