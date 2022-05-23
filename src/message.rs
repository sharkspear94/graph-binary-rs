use std::collections::HashMap;

use serde::de::Visitor;
use serde::Deserialize;
use uuid::Uuid;

use crate::graph_binary::{Decode, Encode, GraphBinary, MapKeys, VALUE_PRESENT};
use crate::specs::{CORE_TYPE_BINDING, CORE_TYPE_INT, CORE_TYPE_STRING};

use super::graph_binary;
use super::structure::map::Map;

struct Request {
    version: u8,
    request_id: uuid::Uuid,
    op: String,
    processor: String,
    args: Map,
}

impl Request {
    fn build_fq_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(1024);

        buf.push(self.version);
        self.request_id.write_patial_bytes(&mut buf);
        self.op.write_patial_bytes(&mut buf);
        self.processor.write_patial_bytes(&mut buf);
        self.args.write_patial_bytes(&mut buf);

        buf
    }
}

#[test]
fn request_message_test() {
    let msg = [
        0x81, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
        0xee, 0xff, 0x00, 0x00, 0x00, 0x04, b'e', b'v', b'a', b'l', 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x03, 0x03, 0x0, 0x00, 0x00, 0x00, 0x07, b'g', b'r', b'e', b'm', b'l', b'i',
        b'n', 0x03, 0x0, 0x00, 0x00, 0x00, 0x06, b'g', b'.', b'V', b'(', b'x', b')', 0x03, 0x0,
        0x00, 0x00, 0x00, 0x08, b'l', b'a', b'n', b'g', b'u', b'a', b'g', b'e', 0x03, 0x0, 0x00,
        0x00, 0x00, 0x0e, b'g', b'r', b'e', b'm', b'l', b'i', b'n', b'-', b'g', b'r', b'o', b'o',
        b'v', b'y', 0x3, 0x0, 0x00, 0x00, 0x00, 0x08, b'b', b'i', b'n', b'd', b'i', b'n', b'g',
        b's', 0x14, 0x0, 0x00, 0x00, 0x00, 0x01, b'x', 0x1, 0x0, 0x00, 0x00, 0x00, 0x01,
    ];

    // let mut client = ClientBuilder::new("ws://127.0.0.1:8182/")
    //     .unwrap()
    //     .add_protocol("application/vnd.graphbinary-v1.0")
    //     .connect_insecure()
    //     .unwrap();

    // let (mut rx,mut tx) = client.split().unwrap();

    // tx.send_message(&OwnedMessage::Binary(Vec::from_iter(msg)));

    //println!("{:?}",m);
    let mut args = HashMap::new();

    args.insert(
        MapKeys::String("gremlin".to_string()),
        GraphBinary::String("g.V().hasLabel(x).elementMap().toList()".to_string()),
    );
    args.insert(
        MapKeys::String("language".to_string()),
        GraphBinary::String("gremlin-groovy".to_string()),
    );

    let mut bindings = HashMap::new();
    bindings.insert(
        MapKeys::String("x".to_string()),
        GraphBinary::String("software".to_string()),
    );

    args.insert(
        MapKeys::String("bindings".to_string()),
        GraphBinary::Map(Map { map: bindings }),
    );

    let args = Map { map: args };

    let req = Request {
        version: 0x81,
        request_id: uuid::Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]),
        op: "eval".to_owned(),
        processor: "".to_owned(),
        args,
    };

    let mut buf: Vec<u8> = vec![];
    let mime_type = "application/vnd.graphbinary-v1.0";

    buf.push(mime_type.len() as u8);
    buf.extend(mime_type.as_bytes());
    buf.extend(req.build_fq_bytes());
    println!("printing: {:?}", buf);
    // assert_eq!(msg,req.build_fq_bytes()[..])
}

#[derive(Debug)]
struct Response {
    version: u8,
    request_id: Option<uuid::Uuid>,
    status_code: i32,
    status_message: Option<String>,
    status_attribute: HashMap<GraphBinary, GraphBinary>,
    result_meta: HashMap<GraphBinary, GraphBinary>,
    result_data: GraphBinary,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            version: 0x81,
            request_id: None,
            status_code: 200,
            status_message: None,
            status_attribute: HashMap::new(),
            result_meta: HashMap::new(),
            result_data: GraphBinary::UnspecifiedNullObject,
        }
    }
}

impl Response {
    fn new() -> Self {
        Response {
            version: u8::default(),
            request_id: None,
            status_code: i32::default(),
            status_message: None,
            status_attribute: HashMap::new(),
            result_meta: HashMap::new(),
            result_data: GraphBinary::UnspecifiedNullObject,
        }
    }

    pub fn builder() -> ResponseBuilder {
        ResponseBuilder {
            resp: Response::new(),
        }
    }
}

#[test]
fn test() {
    let response = Response::builder()
        .with_request_id(None)
        .status_code(200)
        .with_status_message(None)
        .status_attribute(HashMap::new())
        .result_meta(HashMap::new())
        .result_data(42.into())
        .build();
}

#[derive(Debug)]
struct ResponseBuilder {
    resp: Response,
}

impl ResponseBuilder {
    pub fn with_version(mut self, version: u8) -> ResponseBuilder {
        self.resp.version = version;
        self
    }

    pub fn with_request_id(mut self, request_id: Option<Uuid>) -> ResponseBuilder {
        self.resp.request_id = request_id;
        self
    }

    pub fn status_code(mut self, status_code: i32) -> ResponseBuilder {
        self.resp.status_code = status_code;
        self
    }

    pub fn with_status_message(mut self, status_message: Option<String>) -> ResponseBuilder {
        self.resp.status_message = status_message;
        self
    }

    pub fn status_attribute(
        mut self,
        status_attribute: HashMap<GraphBinary, GraphBinary>,
    ) -> ResponseBuilder {
        self.resp.status_attribute = status_attribute;
        self
    }

    pub fn result_meta(
        mut self,
        result_meta: HashMap<GraphBinary, GraphBinary>,
    ) -> ResponseBuilder {
        self.resp.result_meta = result_meta;
        self
    }

    fn result_data(mut self, result_data: GraphBinary) -> ResponseBuilder {
        self.resp.result_data = result_data;
        self
    }

    fn build(self) -> Response {
        self.resp
    }
}

impl<'de> Deserialize<'de> for Response {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(ResponseVisitor)
    }
}

struct ResponseVisitor;

impl<'de> Visitor<'de> for ResponseVisitor {
    type Value = Response;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a struct Response")
    }

    fn visit_bytes<E>(self, mut v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // let version = u8::partial_decode(&mut v);
        // let uuid = Uuid::partial_decode(&mut v);
        // let status_code = i32::partial_decode(&mut v);
        // let status_message = String::partial_decode(&mut v);
        // let status_attributes = HashMap::<MapKeys, GraphBinary>::partial_decode(&mut v);
        // let result_meta = HashMap::<MapKeys, GraphBinary>::partial_decode(&mut v);
        // let result_data = GraphBinary::fully_self_decode(&mut v)?;

        Ok(Response::new())
    }
}

#[test]
fn print_msg() {
    let mut args = HashMap::new();

    args.insert(
        MapKeys::String("gremlin".to_string()),
        GraphBinary::String(
            "g.E().hasLabel('test').subgraph('subGraph').cap('subGraph')".to_string(),
        ),
    );
    args.insert(
        MapKeys::String("language".to_string()),
        GraphBinary::String("gremlin-groovy".to_string()),
    );

    // let mut bindings = HashMap::new();
    // bindings.insert(
    //     MapKeys::String("x".to_string()),
    //     GraphBinary::String("1".to_string()),
    // );

    args.insert(
        MapKeys::String("bindings".to_string()),
        GraphBinary::Map(Map {
            map: HashMap::new(),
        }),
    );

    let args = Map { map: args };

    let req = Request {
        version: 0x81,
        request_id: uuid::Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]),
        op: "eval".to_owned(),
        processor: "".to_owned(),
        args,
    };

    let mut buf: Vec<u8> = vec![];
    let mime_type = "application/vnd.graphbinary-v1.0";

    buf.push(mime_type.len() as u8);
    buf.extend(mime_type.as_bytes());
    buf.extend(req.build_fq_bytes());
    println!("printing: {:?}", buf);
}
