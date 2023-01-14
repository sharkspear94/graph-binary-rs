use thiserror::{self, Error};

#[derive(Error, Debug)]
pub enum GremlinError {
    #[error("reading from Reader")]
    Decode(#[from] tinkerpop_io::error::DecodeError),
    #[error("reading from Reader")]
    Encode(#[from] tinkerpop_io::error::EncodeError),
    #[error("reading from Reader")]
    GraphSon(#[from] tinkerpop_io::error::GraphSonError),
}
