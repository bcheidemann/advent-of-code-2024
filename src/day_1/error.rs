use std::{error::Error, fmt::Display};

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidRow { row: String },
    InvalidLocationId { location_id: String, reason: String },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidRow { row } => write!(f, "Invalid row ({row})"),
            ParseError::InvalidLocationId {
                location_id,
                reason,
            } => {
                write!(f, "Invalid location id ({location_id}): {reason}")
            }
        }
    }
}

impl From<ParseError> for AnalyzeError {
    fn from(value: ParseError) -> Self {
        AnalyzeError::ParseError(value)
    }
}

impl Error for ParseError {}

pub type AnalyzeResult<T> = Result<T, AnalyzeError>;

#[derive(Debug, PartialEq)]
pub enum AnalyzeError {
    ParseError(ParseError),
    IntegerOverflow,
}

impl Display for AnalyzeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalyzeError::IntegerOverflow => write!(f, "Integer overflow"),
            AnalyzeError::ParseError(err) => write!(f, "Parse error: {err}"),
        }
    }
}

impl Error for AnalyzeError {}
