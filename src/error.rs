use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid argument: {0}")]
    InvalidInput(String),

    #[error(transparent)]
    Win32(#[from] d3d11_ffi::core::Error),

    #[error("The requested interface is not supported by the device")]
    InterfaceUnsupported,
}

// validate_input!(condition, error_format, ...)
macro_rules! validate_input {
    ($condition:expr, $error_format:literal $(, $arg:expr)*) => {
        if !$condition {
            return Err($crate::error::Error::InvalidInput(format!($error_format, $($arg),*)));
        }
    };
}

macro_rules! bail {
    ($format:literal $(, $arg:expr)*) => {
        return Err($crate::error::Error::InvalidInput(format!($format, $($arg),*)));
    };
}

pub(crate) use bail;
pub(crate) use validate_input;
