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

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::new(ErrorKind::ParseIntError(e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::new(ErrorKind::SerdeJsonError(e))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::new(ErrorKind::StdIoError(e))
    }
}

#[cfg(feature = "not-wasm")]
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::new(ErrorKind::RqwestError(e))
    }
}


#[cfg(feature = "not-wasm")]
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


#[cfg(feature = "wasm")]
impl From<gloo_net::http::Response> for Error {
    fn from(e: gloo_net::http::Response) -> Self {
        Error::new(ErrorKind::GlooNetworkError(e))
    }
}

#[cfg(feature = "wasm")]
impl From<serde_urlencoded::ser::Error> for Error {
    fn from(e: serde_urlencoded::ser::Error) -> Self {
        Error::new(ErrorKind::SerdeUrlencodedError(e))
    }
}

pub enum ErrorKind {
    OsynicOsuApiV2Error(String),
    SerdeJsonError(serde_json::Error),
    ParseIntError(std::num::ParseIntError),
    StdIoError(std::io::Error),
    #[cfg(feature = "not-wasm")]
    RqwestError(reqwest::Error),
    #[cfg(feature = "not-wasm")]
    NetworkError(reqwest::Response),
    #[cfg(feature = "wasm")]
    GlooError(gloo_net::Error),
    #[cfg(feature = "wasm")]
    GlooNetworkError(gloo_net::http::Response),
    #[cfg(feature = "wasm")]
    SerdeUrlencodedError(serde_urlencoded::ser::Error),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::OsynicOsuApiV2Error(e) => write!(f, "OsynicOsuApiV2Error: {}", e),
            ErrorKind::ParseIntError(e) => write!(f, "std::num::ParseIntError: {}", e),
            ErrorKind::StdIoError(e) => write!(f, "std::io::Error: {}", e),
            ErrorKind::SerdeJsonError(e) => write!(f, "serde_json::Error: {}", e),
            #[cfg(feature = "not-wasm")]
            ErrorKind::RqwestError(e) => write!(f, "reqwest::Error: {}", e),
            #[cfg(feature = "not-wasm")]
            ErrorKind::NetworkError(e) => write!(f, "NetworkError: {:?}", e),
            #[cfg(feature = "wasm")]
            ErrorKind::GlooError(e) => write!(f, "gloo_net::Error: {}", e),
            #[cfg(feature = "wasm")]
            ErrorKind::GlooNetworkError(e) => write!(f, "GlooNetworkError: {:?}", e),
            #[cfg(feature = "wasm")]
            ErrorKind::SerdeUrlencodedError(e) => write!(f, "serde_urlencoded::Error: {}", e),
        }
    }
}

impl std::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::OsynicOsuApiV2Error(e) => write!(f, "OsynicOsuApiV2Error: {}", e),
            ErrorKind::StdIoError(e) => write!(f, "std::io::Error: {:?}", e),
            ErrorKind::ParseIntError(e) => write!(f, "std::num::ParseIntError: {:?}", e),
            ErrorKind::SerdeJsonError(e) => write!(f, "serde_json::Error: {:?}", e),
            #[cfg(feature = "not-wasm")]
            ErrorKind::RqwestError(e) => write!(f, "reqwest::Error: {:?}", e),
            #[cfg(feature = "not-wasm")]
            ErrorKind::NetworkError(e) => write!(f, "NetworkError: {:?}", e),
            #[cfg(feature = "wasm")]
            ErrorKind::GlooError(e) => write!(f, "gloo_net::Error: {}", e),
            #[cfg(feature = "wasm")]
            ErrorKind::GlooNetworkError(e) => write!(f, "GlooNetworkError: {:?}", e),
            #[cfg(feature = "wasm")]
            ErrorKind::SerdeUrlencodedError(e) => write!(f, "serde_urlencoded::Error: {:?}", e),
        }
    }
}
