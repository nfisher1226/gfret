use {
    std::{error, fmt, io},
    toml::de,
    toml::ser,
};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    TomlDecode(de::Error),
    TomlEncode(ser::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<de::Error> for Error {
    fn from(e: de::Error) -> Self {
        Self::TomlDecode(e)
    }
}

impl From<ser::Error> for Error {
    fn from(e: ser::Error) -> Self {
        Self::TomlEncode(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "{e}"),
            Self::TomlDecode(e) => write!(f, "{e}"),
            Self::TomlEncode(e) => write!(f, "{e}"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::IoError(e) => Some(e),
            Self::TomlDecode(e) => Some(e),
            Self::TomlEncode(e) => Some(e),
        }
    }
}
