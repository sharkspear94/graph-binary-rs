use thiserror::{self, Error};

#[derive(Error, Debug)]
pub enum GremlinError {
    #[error("reading from Reader")]
    Decode(#[from] gremlin_types::error::DecodeError),
    #[error("reading from Reader")]
    Encode(#[from] gremlin_types::error::EncodeError),
    #[error("reading from Reader")]
    GraphSon(#[from] gremlin_types::error::GraphSonError),
}
