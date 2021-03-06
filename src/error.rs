use serde::ser;
use std::{io, num::TryFromIntError, str::Utf8Error, string::FromUtf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("writing into Writer")]
    IoError(#[from] io::Error),

    #[error("serialiezing")]
    SerilizationError(String),

    #[error("try from int error")]
    TryError(#[from] TryFromIntError),
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("reading from Reader")]
    IoError(#[from] io::Error),

    #[error("decoding type `{0}`")]
    DecodeError(String),

    #[error("converting bytes to utf8")]
    Utf8ErrorString(#[from] FromUtf8Error),

    #[error("converting bytes to utf8")]
    Utf8Error(#[from] Utf8Error),

    #[error("converting from u8 to `{0}`")]
    ConvertError(String),

    #[error("serialiezing")]
    DeserilizationError(String),

    #[error("try from slice error")]
    SliceError(#[from] std::array::TryFromSliceError),

    #[error("try from int error")]
    TryError(#[from] TryFromIntError),
}

impl ser::Error for EncodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        EncodeError::SerilizationError(msg.to_string())
    }
}

impl serde::de::Error for DecodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        DecodeError::DeserilizationError(msg.to_string())
    }
}
