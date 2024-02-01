/// A type that can be used to represent a result that is always `Ok`
/// 
/// ### O_O
///
/// It is a shorthand for `Result<(), Error>`.
pub const IO_OK: Result = Ok(());

/// An error that occurs when reading from or writing to a hardware component
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    /// The device was disconnected
    DeviceDisconnected,
    /// The device was not found
    DeviceNotFound,
    /// The method was not implemented
    /// 
    /// Or a stub was called
    MethodNotImplemented,
    /// Some other error occurred
    Other {
        /// The error message
        message: &'static str,
    },
}

impl Error {
    /// Creates a new `Error::Other` with the given message
    #[inline]
    #[must_use = "This returns a new Error"]
    pub const fn new(message: &'static str) -> Self {
        Self::Other { message }
    }
    /// Returns the error message
    #[inline]
    #[must_use = "This returns a new string slice"]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Error::DeviceDisconnected => "Device disconnected",
            Error::DeviceNotFound => "Device not found",
            Error::MethodNotImplemented => "Method not implemented",
            Error::Other { message } => message,
        }
    }
}

/// A result that occurs when reading or writing to a hardware component
pub type Result<T = (), E = Error> = core::result::Result<T, E>;

impl core::fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&'static str> for Error {
    #[inline]
    fn from(message: &'static str) -> Self {
        Self::Other { message }
    }
}

impl From<Error> for String {
    #[inline]
    fn from(error: Error) -> Self {
        format!("{}", error)
    }
}

impl From<Error> for &'static str {
    #[inline]
    fn from(error: Error) -> Self {
        error.as_str()
    }
}

#[allow(unsafe_code)]
unsafe impl Send for Error {}
#[allow(unsafe_code)]
unsafe impl Sync for Error {}

impl std::error::Error for Error {}

#[cfg(feature = "axum")]
impl From<::axum::Error> for Error {
    #[inline]
    fn from(_: ::axum::Error) -> Self {
        Self::Other {
            message: "Axum error occurred",
        }
    }
}

impl From<::std::io::Error> for Error {
    #[inline]
    fn from(error: ::std::io::Error) -> Self {
        match error.kind() {
            ::std::io::ErrorKind::NotFound => Self::DeviceNotFound,
            ::std::io::ErrorKind::BrokenPipe => Self::DeviceDisconnected,
            _ => Self::Other {
                message: "IO error occurred",
            },
        }
    }
}
