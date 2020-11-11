use thiserror::Error;

use nom::error::{ErrorKind as NomErrorKind, ParseError};
use nom::IResult as NomIResult;

pub type Result<T> = std::result::Result<T, ErrorKind>;
pub type IResult<I, T> = NomIResult<I, T, ErrorKind>;

#[derive(Error, Debug, PartialEq)]
pub enum ErrorKind {
    #[error("Parse error: {}", msg)]
    ParseError { msg: String },
}

impl ErrorKind {
    pub(crate) fn from_nom_error(msg: impl ToString) -> ErrorKind {
        ErrorKind::ParseError {
            msg: msg.to_string(),
        }
    }
}

impl<I> ParseError<I> for ErrorKind {
    fn from_error_kind(_input: I, kind: NomErrorKind) -> Self {
        ErrorKind::ParseError {
            msg: format!("{:?}", kind),
        }
    }

    fn append(_: I, _: NomErrorKind, other: Self) -> Self {
        other
    }
}
