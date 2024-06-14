use std::fmt;

use miette::Diagnostic;
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

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::DeJson(err) => Some(err),
        }
    }
}

impl Diagnostic for Error {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        match self {
            Self::Io(_err) => Some(Box::new("kx_lsm::io_error")),
            Self::DeJson(_err) => Some(Box::new("kx_lsm::dejson_error")),
        }
    }

    // fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
    //     match self {
    //         Self::Io(err) => Some(err),
    //         Self::DeJson(err) => Some(err),
    //     }
    // }
}
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
