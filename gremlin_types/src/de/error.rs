use serde::ser;
use std::{io, num::TryFromIntError, str::Utf8Error, string::FromUtf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphSONv3Error {
    #[error("reading from Reader")]
    Json(#[from] serde_json::Error),

    #[error("reading from Reader")]
    SerdeDeserilization(String),
}

impl serde::de::Error for GraphSONv3Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        GraphSONv3Error::SerdeDeserilization(msg.to_string())
    }
}
