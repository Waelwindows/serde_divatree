use ::serde::{de, ser};
use thiserror::*;

use std::fmt::{self, Display};
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug, Default, Error, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[error("Syntax error in line {line_num}. `{line}`")]
pub struct ParseError {
    pub line_num: usize,
    pub line: String,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DeserializerError {
    #[error("An internal parser error occured.")]
    ParseError(#[from] ParseError),
    #[error("Expected a value node. Found a key node instead.")]
    ExpectedValueNode,
    #[error("Expected a key node. Found a value node instead.")]
    ExpectedKeyNode,
    #[error("Expected an integer, found something else")]
    ExpectedInteger(#[from] ParseIntError),
    #[error("Expected an float, found something else")]
    ExpectedFloat(#[from] ParseFloatError),
    #[error("Expected a sequence, found something else")]
    ExpectedSequenece,
    #[error("Expected a character, found something else")]
    ExpectedChar,
    #[error("Expected a boolean, found something else")]
    ExpectedBool,
    #[error("Expected the start of a tuple, found something else")]
    ExpectedTuple,
    #[error("Expected a non empty tuple, found an empty tuple")]
    ExpectedNonEmptyTuple,
    #[error("Expected the end of a tuple, found something else")]
    ExpectedTupleEnd,
    #[error("{0}")]
    Custom(String),
}

impl de::Error for DeserializerError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}
