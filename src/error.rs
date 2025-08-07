use std::ffi::CStr;

use crate::bindings::vips_error_buffer;

#[derive(Debug)]
pub enum Error {
    InitializationError(String),
    IOError(String),
    OperationError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InitializationError(msg) => {
                write!(
                    f,
                    "vips error: InitializationError - {}",
                    msg
                )
            }
            Error::OperationError(msg) => write!(
                f,
                "vips error: OperationError - {}",
                msg
            ),
            Error::IOError(msg) => write!(
                f,
                "vips error: IOError - {}",
                msg
            ),
        }
    }
}

impl Error {
    pub(crate) fn extend(self) -> Self {
        let erro_buffer = unsafe { vips_error_buffer() };
        if erro_buffer.is_null() {
            return self;
        }

        if let Ok(detail) = unsafe { CStr::from_ptr(erro_buffer).to_str() } {
            match self {
                Error::InitializationError(msg) => Error::InitializationError(format!(
                    "{}. {}",
                    msg, detail
                )),
                Error::OperationError(msg) => Error::OperationError(format!(
                    "{}. {}",
                    msg, detail
                )),
                Error::IOError(msg) => Error::IOError(format!(
                    "{}. {}",
                    msg, detail
                )),
            }
        } else {
            self
        }
    }
}

impl std::error::Error for Error {}
