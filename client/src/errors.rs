use std::ffi::NulError;

use sdl2::{IntegerOrSdlError, video::WindowBuildError};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    StringError(String),
    SDLErrorWindowBuild(WindowBuildError),
    SDLCanvasError(IntegerOrSdlError),
    GLNullError(NulError),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl From<NulError> for Error {
    fn from(value: NulError) -> Self {
        Self::GLNullError(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::StringError(value)
    }
}

impl From<WindowBuildError> for Error {
    fn from(value: WindowBuildError) -> Self {
        Self::SDLErrorWindowBuild(value)
    }
}
impl From<IntegerOrSdlError> for Error {
    fn from(value: IntegerOrSdlError) -> Self {
        Self::SDLCanvasError(value)
    }
}

impl std::error::Error for Error {}
