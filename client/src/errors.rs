pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    StringError(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::StringError(value)
    }
}

impl std::error::Error for Error {}
