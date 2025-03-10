/// Error handling for the all module.
pub type Result<T> = core::result::Result<T, Error>;

pub struct Error {
    inner: Box<ErrorKind>,
}

impl Error {
    pub fn new(inner: ErrorKind) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self::new(kind)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::new(ErrorKind::OsynicOsuApiV2Error(e))
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Error::new(ErrorKind::OsynicOsuApiV2Error(e.to_string()))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::new(ErrorKind::StdIoError(e))
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::new(ErrorKind::RqwestError(e))
    }
}

impl From<reqwest::Response> for Error {
    fn from(e: reqwest::Response) -> Self {
        Error::new(ErrorKind::NetworkError(e))
    }
}

#[cfg(feature = "wasm")]
impl From<gloo_net::Error> for Error {
    fn from(e: gloo_net::Error) -> Self {
        Error::new(ErrorKind::GlooError(e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::new(ErrorKind::SerdeJsonError(e))
    }
}

pub enum ErrorKind {
    OsynicOsuApiV2Error(String),
    NetworkError(reqwest::Response),
    SerdeJsonError(serde_json::Error),
    StdIoError(std::io::Error),
    RqwestError(reqwest::Error),
    #[cfg(feature = "wasm")]
    GlooError(gloo_net::Error),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::OsynicOsuApiV2Error(e) => write!(f, "OsynicOsuApiV2Error: {}", e),
            ErrorKind::NetworkError(e) => write!(f, "NetworkError: {:?}", e),
            ErrorKind::StdIoError(e) => write!(f, "std::io::Error: {}", e),
            ErrorKind::SerdeJsonError(e) => write!(f, "serde_json::Error: {}", e),
            ErrorKind::RqwestError(e) => write!(f, "reqwest::Error: {}", e),
            #[cfg(feature = "wasm")]
            ErrorKind::GlooError(e) => write!(f, "gloo_net::Error: {}", e),
        }
    }
}

impl std::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::OsynicOsuApiV2Error(e) => write!(f, "OsynicOsuApiV2Error: {}", e),
            ErrorKind::NetworkError(e) => write!(f, "NetworkError: {:?}", e),
            ErrorKind::StdIoError(e) => write!(f, "std::io::Error: {:?}", e),
            ErrorKind::SerdeJsonError(e) => write!(f, "serde_json::Error: {:?}", e),
            ErrorKind::RqwestError(e) => write!(f, "reqwest::Error: {:?}", e),
            #[cfg(feature = "wasm")]
            ErrorKind::GlooError(e) => write!(f, "gloo_net::Error: {}", e),
        }
    }
}
