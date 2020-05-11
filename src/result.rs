//! Error types that can be emitted from this library

use std::{convert, error, fmt, io};

/// Generic result type with ZipError as its error variant
pub type ZipResult<T> = Result<T, ZipError>;

/// Error type for Zip
#[derive(Debug)]
pub enum ZipError {
    /// An Error caused by I/O
    Io(io::Error),

    /// This file is probably not a zip archive
    InvalidArchive(&'static str),

    /// This archive is not supported
    UnsupportedArchive(&'static str),

    /// The requested file could not be found in the archive
    FileNotFound,
}

impl ZipError {
    fn detail(&self) -> ::std::borrow::Cow<'_, str> {
        match *self {
            ZipError::Io(ref io_err) => {
                (String::from("Io Error: ") + &(io_err as &dyn error::Error).to_string()).into()
            }
            ZipError::InvalidArchive(msg) | ZipError::UnsupportedArchive(msg) => {
                (String::from("Invalid/unsupported archive: ") + msg).into()
            }
            ZipError::FileNotFound => self.to_string().into(),
        }
    }
}

impl convert::From<io::Error> for ZipError {
    fn from(err: io::Error) -> ZipError {
        ZipError::Io(err)
    }
}

impl convert::From<ZipError> for io::Error {
    fn from(err: ZipError) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

impl fmt::Display for ZipError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt.write_str(&*self.detail())
    }
}

impl error::Error for ZipError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            ZipError::Io(ref io_err) => Some(io_err as &dyn error::Error),
            _ => None,
        }
    }
}
