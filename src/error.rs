use std::fmt;

use smol::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    DeJson(nanoserde::DeJsonErr),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(_err) => f.write_str("IO error"),
            Self::DeJson(_err) => f.write_str("DeJson error"),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<nanoserde::DeJsonErr> for Error {
    fn from(value: nanoserde::DeJsonErr) -> Self {
        Self::DeJson(value)
    }
}
