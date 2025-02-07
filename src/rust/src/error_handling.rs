use extendr_api::prelude::Error;
use toml_edit::TomlError;

#[derive(Debug)]
pub(crate) enum TomlEditRError {
    TomlError(TomlError),
    ExtendrError(Error),
    OtherError(Box<dyn std::error::Error>),
    CrateError(String),
}

impl From<TomlEditRError> for Error {
    fn from(value: TomlEditRError) -> Self {
        match value {
            TomlEditRError::TomlError(e) => Error::Other(e.to_string()),
            TomlEditRError::ExtendrError(e) => e,
            TomlEditRError::OtherError(e) => Error::Other(e.to_string()),
            TomlEditRError::CrateError(e) => Error::Other(e),
        }
    }
}

impl From<TomlError> for TomlEditRError {
    fn from(value: TomlError) -> Self {
        TomlEditRError::TomlError(value)
    }
}

impl From<Error> for TomlEditRError {
    fn from(value: Error) -> Self {
        TomlEditRError::ExtendrError(value)
    }
}
