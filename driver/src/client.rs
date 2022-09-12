use std::{io::Read, net::IpAddr};

use r2d2::{ManageConnection, Pool};
use websocket::{
    sync::{
        stream::{NetworkStream, TcpStream},
        Client, Stream,
    },
    ClientBuilder, Message,
};

use crate::error::GremlinError;

pub struct Connection<S: NetworkStream>(S);

pub struct GClient {
    connection: Box<dyn NetworkStream + Send>,
    options: (),
}

impl GClient {
    pub fn new(ip: &str) -> Self {
        let connection = TcpStream::connect(ip).unwrap();
        GClient {
            connection,
            options: (),
        }
    }
}

pub struct GClientBuilder {
    client: GClient,
}

impl GClientBuilder {
    pub fn alias(&mut self, alias: &str) -> &mut Self {
        self
    }

    pub fn pool_size(&mut self) -> &mut Self {
        self
    }

    pub fn connect(self) -> GClient {
        self.client
    }
}

#[test]
fn test() {
    let mut client = GClient::new("ws://localhost:8182/gremlin");
}
