use std::collections::HashMap;
use std::fmt::Display;
use std::vec;

use serde::de::{IntoDeserializer, Visitor};
use serde::Deserialize;
use uuid::Uuid;

use crate::de::from_slice;
use crate::error::{DecodeError, EncodeError};
use crate::graph_binary::{Decode, Encode, GraphBinary, MapKeys};
use crate::structure::bytecode::{ByteCode, Step};
use crate::structure::enums::T;
use crate::structure::lambda::Lambda;

#[derive(Debug, PartialEq)]
pub struct Request {
    version: u8,
    request_id: uuid::Uuid,
    op: String,
    processor: String,
    args: HashMap<MapKeys, GraphBinary>,
}

impl Request {
    pub fn write_gb_bytes<W: std::io::Write>(
        &self,
        writer: &mut W,
        mime_type: &str,
    ) -> Result<(), EncodeError> {
        writer.write_all(&[mime_type.len() as u8])?;
        writer.write_all(mime_type.as_bytes())?;
        self.encode(writer)
    }
}

impl Default for Request {
    fn default() -> Self {
        Self {
            version: 0x81,
            request_id: Uuid::new_v4(),
            op: "".to_owned(),
            processor: String::default(),
            args: HashMap::from([("language".into(), "gremlin-groovy".into())]),
        }
    }
}

impl Encode for Request {
    fn type_code() -> u8 {
        unimplemented!("not supported for Request")
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        _writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        unimplemented!("")
    }
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.version.partial_encode(writer)?;
        self.request_id.partial_encode(writer)?;
        self.op.partial_encode(writer)?;
        self.processor.partial_encode(writer)?;
        self.args.partial_encode(writer)
    }
}
impl Request {
    pub fn builder() -> RequestBuilder {
        RequestBuilder(Request::default())
    }
}

pub struct RequestBuilder(Request);

impl RequestBuilder {
    pub fn version(mut self, version: u8) -> Self {
        self.0.version = version;
        self
    }
    pub fn request_id(mut self, request_id: Uuid) -> Self {
        self.0.request_id = request_id;
        self
    }
    pub fn session(mut self, session_identifier: &str) -> Self {
        self.0.processor = "session".to_owned();
        self.0
            .args
            .insert("session".into(), session_identifier.into());
        self
    }
    pub fn op(mut self, op: &str) -> Self {
        self.0.op = op.to_owned();
        self
    }
    pub fn processor(mut self, processor: &str) -> Self {
        self.0.processor = processor.to_owned();
        self
    }
    pub fn authentication(mut self) -> AuthRequestBuilder {
        self.0.op = "authentication".to_owned();
        self.0.processor = "".to_owned();
        self.0.args.insert("saslMechanism".into(), "PLAIN".into());
        AuthRequestBuilder(self.0)
    }
    pub fn eval(mut self) -> EvalBuilder {
        self.0.op = "eval".to_owned();
        self.0
            .args
            .insert("language".into(), "gremlin-groovy".into());
        EvalBuilder(self.0)
    }
    pub fn bytecode(mut self) -> BytecodeBuilder {
        self.0.op = "bytecode".to_owned();
        self.0.processor = "traversal".to_owned();
        self.0.args.insert(
            MapKeys::String("aliases".to_string()),
            GraphBinary::Map(HashMap::from([("g".into(), "g".into())])),
        );
        BytecodeBuilder(self.0)
    }
    pub fn close(mut self, session_identifier: &str) -> Request {
        self.0.op = "close".into();
        self.0.processor = "session".to_owned();
        self.0
            .args
            .insert("session".into(), session_identifier.into());
        self.0
    }
}

pub struct BytecodeBuilder(Request);

impl BytecodeBuilder {
    pub fn gremlin(mut self, bytecode: ByteCode) -> Self {
        self.0.args.insert("gremlin".into(), bytecode.into());
        self
    }
    pub fn aliases(mut self, aliases: HashMap<String, String>) -> Self {
        self.0.args.insert("aliases".into(), aliases.into());
        self
    }
    pub fn build(self) -> Request {
        self.0
    }
}

pub struct EvalBuilder(Request);

impl EvalBuilder {
    pub fn bindings(mut self, bindings: HashMap<String, GraphBinary>) -> Self {
        self.0.args.insert("bindings".into(), bindings.into());
        self
    }
    pub fn gremlin(mut self, script: &str) -> Self {
        self.0.args.insert("gremlin".into(), script.into());
        self
    }
    pub fn session(mut self, session_identifier: &str) -> Self {
        self.0.processor = "session".to_owned();
        self.0
            .args
            .insert("session".into(), session_identifier.into());
        self
    }
    pub fn aliases(mut self, aliases: HashMap<String, String>) -> Self {
        if let Some(GraphBinary::Map(map)) = self.0.args.get_mut(&"aliases".into()) {
            map.extend(aliases.into_iter().map(|(k, v)| (k.into(), v.into())));
        } else {
            self.0.args.insert("aliases".into(), aliases.into());
        }
        self
    }
    pub fn alias(mut self, source: &str, alias: &str) -> Self {
        if let Some(GraphBinary::Map(map)) = self.0.args.get_mut(&"aliases".into()) {
            map.insert(source.into(), alias.into());
        } else {
            self.0.args.insert(
                "aliases".into(),
                HashMap::<MapKeys, GraphBinary>::from([(source.into(), alias.into())]).into(),
            );
        }
        self
    }
    pub fn language(mut self, language: &str) -> Self {
        self.0.args.insert("language".into(), language.into());
        self
    }
    pub fn build(self) -> Request {
        self.0
    }
}

pub struct AuthRequestBuilder(Request);

impl AuthRequestBuilder {
    pub fn sasl_mechanism(mut self, mechanism: &str) -> Self {
        self.0.args.insert("saslMechanism".into(), mechanism.into());
        self
    }

    pub fn sasl(mut self, sasl: &str) -> Self {
        self.0.args.insert("sasl".into(), sasl.into());
        self
    }
    pub fn session(mut self, session_identifier: &str) -> Self {
        self.0.processor = "session".to_owned();
        self.0
            .args
            .insert("session".into(), session_identifier.into());
        self
    }
    pub fn build(self) -> Request {
        self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct Response {
    version: u8,
    request_id: Option<uuid::Uuid>,
    status_code: i32,
    status_message: Option<String>,
    status_attribute: HashMap<MapKeys, GraphBinary>,
    result_meta: HashMap<MapKeys, GraphBinary>,
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

    pub fn result_data(&self) -> &GraphBinary {
        &self.result_data
    }

    pub fn unwind_traverser(&self) -> Result<Vec<&GraphBinary>, DecodeError> {
        match &self.result_data {
            GraphBinary::List(l) => Ok(l
                .iter()
                .filter_map(|g| match g {
                    GraphBinary::Traverser(t) => Some(t),
                    _ => None,
                })
                .flat_map(|f| f.iter())
                // .cloned()
                .collect()),
            _ => Err(DecodeError::DecodeError(
                "expected list in unwinding result data".to_string(),
            )),
        }
    }
}

#[derive(Debug)]
pub struct ResponseBuilder {
    resp: Response,
}

impl ResponseBuilder {
    pub fn version(mut self, version: u8) -> ResponseBuilder {
        self.resp.version = version;
        self
    }

    pub fn request_id(mut self, request_id: Option<Uuid>) -> ResponseBuilder {
        self.resp.request_id = request_id;
        self
    }

    pub fn status_code(mut self, status_code: i32) -> ResponseBuilder {
        self.resp.status_code = status_code;
        self
    }

    pub fn status_message(mut self, status_message: Option<String>) -> ResponseBuilder {
        self.resp.status_message = status_message;
        self
    }

    pub fn status_attribute(
        mut self,
        status_attribute: HashMap<MapKeys, GraphBinary>,
    ) -> ResponseBuilder {
        self.resp.status_attribute = status_attribute;
        self
    }

    pub fn result_meta(mut self, result_meta: HashMap<MapKeys, GraphBinary>) -> ResponseBuilder {
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

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status_code {
            200 => todo!(),
            500 => {
                writeln!(f, "status Code: 500")?;
                writeln!(f, "request Id: {}", self.request_id.unwrap_or_default())?;
                writeln!(
                    f,
                    "status_message: {}",
                    self.status_message.clone().unwrap_or_default()
                )?;
                let v = self.status_attribute.get(&"exceptions".into()).unwrap();
                writeln!(f, "exceptions: {}", &v.exceptions().unwrap_or_default())?;
                let v = self.status_attribute.get(&"stackTrace".into()).unwrap();
                writeln!(f, "stackTrace : {}", v.get::<String>().unwrap_or_default())
            }
            _ => todo!(),
        }
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
        Response::decode(&mut v)
            .map_err(|err| E::custom(format!("response Visitor Error with Error: {}", err)))
    }
}

impl Decode for Response {
    fn expected_type_code() -> u8 {
        unimplemented!("Response does not have Typecode")
    }

    fn partial_decode<R: std::io::Read>(_reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        unimplemented!("Response can only be decoded with fully_self_decode")
    }

    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let version = u8::partial_decode(reader)?;
        let uuid = Uuid::nullable_decode(reader)?;
        let status_code = i32::partial_decode(reader)?;
        let status_message = String::nullable_decode(reader)?;
        let status_attributes = HashMap::<MapKeys, GraphBinary>::partial_decode(reader)?;
        let result_meta = HashMap::<MapKeys, GraphBinary>::partial_decode(reader)?;
        let result_data = GraphBinary::decode(reader)?;

        Ok(Response::builder()
            .version(version)
            .request_id(uuid)
            .status_code(status_code)
            .status_message(status_message)
            .status_attribute(status_attributes)
            .result_meta(result_meta)
            .result_data(result_data)
            .build())
    }

    fn get_partial_len(_bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        todo!()
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
        b's', 0xa, 0x0, 0x00, 0x00, 0x00, 0x01, 0x03, 0x0, 0x0, 0x0, 0x0, 0x1, b'x', 0x1, 0x0,
        0x00, 0x00, 0x00, 0x01,
    ];
    let mut args = HashMap::new();

    args.insert(
        MapKeys::String("gremlin".to_string()),
        GraphBinary::String("g.V(x)".to_string()),
    );
    args.insert(
        MapKeys::String("language".to_string()),
        GraphBinary::String("gremlin-groovy".to_string()),
    );

    let mut bindings = HashMap::new();
    bindings.insert(MapKeys::String("x".to_string()), 1.into());

    args.insert(
        MapKeys::String("bindings".to_string()),
        GraphBinary::Map(bindings),
    );

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

    req.encode(&mut buf).unwrap();
    assert_eq!(msg.len(), buf.len())
}

#[test]
fn request_message_with_mimetype_test() {
    let msg = [
        0x20, 0x61, 0x70, 0x70, 0x6C, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x2F, 0x76, 0x6E,
        0x64, 0x2E, 0x67, 0x72, 0x61, 0x70, 0x68, 0x62, 0x69, 0x6E, 0x61, 0x72, 0x79, 0x2D, 0x76,
        0x31, 0x2E, 0x30, 0x81, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa,
        0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x00, 0x00, 0x04, b'e', b'v', b'a', b'l', 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x0, 0x00, 0x00, 0x00, 0x07, b'g', b'r', b'e',
        b'm', b'l', b'i', b'n', 0x03, 0x0, 0x00, 0x00, 0x00, 0x06, b'g', b'.', b'V', b'(', b'x',
        b')', 0x03, 0x0, 0x00, 0x00, 0x00, 0x08, b'l', b'a', b'n', b'g', b'u', b'a', b'g', b'e',
        0x03, 0x0, 0x00, 0x00, 0x00, 0x0e, b'g', b'r', b'e', b'm', b'l', b'i', b'n', b'-', b'g',
        b'r', b'o', b'o', b'v', b'y', 0x3, 0x0, 0x00, 0x00, 0x00, 0x08, b'b', b'i', b'n', b'd',
        b'i', b'n', b'g', b's', 0xa, 0x0, 0x00, 0x00, 0x00, 0x01, 0x03, 0x0, 0x0, 0x0, 0x0, 0x1,
        b'x', 0x1, 0x0, 0x00, 0x00, 0x00, 0x01,
    ];
    let mut args = HashMap::new();

    args.insert(
        MapKeys::String("gremlin".to_string()),
        GraphBinary::String("g.V(x)".to_string()),
    );
    args.insert(
        MapKeys::String("language".to_string()),
        GraphBinary::String("gremlin-groovy".to_string()),
    );

    let mut bindings = HashMap::new();
    bindings.insert(MapKeys::String("x".to_string()), 1.into());

    args.insert(
        MapKeys::String("bindings".to_string()),
        GraphBinary::Map(bindings),
    );

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

    req.write_gb_bytes(&mut buf, "application/vnd.graphbinary-v1.0")
        .unwrap();
    assert_eq!(msg.len(), buf.len())
}

#[test]
fn test_respose() {
    let bytes = vec![
        0x81, 0x0, 0x0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc,
        0xdd, 0xee, 0xff, 0x0, 0x0, 0x0, 0xc8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3,
        0x0, 0x0, 0x0, 0x0, 0x4, 0x68, 0x6f, 0x73, 0x74, 0x3, 0x0, 0x0, 0x0, 0x0, 0x10, 0x2f, 0x31,
        0x32, 0x37, 0x2e, 0x30, 0x2e, 0x30, 0x2e, 0x31, 0x3a, 0x31, 0x32, 0x33, 0x34, 0x35, 0x0,
        0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x1, 0x1, 0x0, 0x0, 0x0, 0x0, 0x1d,
    ];

    let resp = Response::decode(&mut &*bytes).unwrap();

    let expected = Response::builder()
        .version(0x81)
        .request_id(Some(Uuid::from_bytes([
            0x0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ])))
        .status_code(200)
        .status_message(Some("".to_owned()))
        .status_attribute(HashMap::from([("host".into(), "/127.0.0.1:12345".into())]))
        .result_meta(HashMap::new())
        .result_data(vec![29_i32].into())
        .build();

    assert_eq!(expected, resp)
}

#[test]
fn test_respose_with_t() {
    let bytes = vec![
        0x81, 0x0, 0x0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc,
        0xdd, 0xee, 0xff, 0x0, 0x0, 0x0, 0xc8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3,
        0x0, 0x0, 0x0, 0x0, 0x4, 0x68, 0x6f, 0x73, 0x74, 0x3, 0x0, 0x0, 0x0, 0x0, 0x10, 0x2f, 0x31,
        0x32, 0x37, 0x2e, 0x30, 0x2e, 0x30, 0x2e, 0x31, 0x3a, 0x31, 0x32, 0x33, 0x34, 0x35, 0x0,
        0x0, 0x0, 0x0, 0x20, 0x0, 0x03, 0x0, 0x0, 0x0, 0x0, 0x2, b'i', b'd',
    ];

    let resp = from_slice(&bytes).unwrap();

    let expected = Response::builder()
        .version(0x81)
        .request_id(Some(Uuid::from_bytes([
            0x0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ])))
        .status_code(200)
        .status_message(Some("".to_owned()))
        .status_attribute(HashMap::from([("host".into(), "/127.0.0.1:12345".into())]))
        .result_meta(HashMap::new())
        .result_data(T::Id.into())
        .build();

    assert_eq!(expected, resp)
}

#[test]
fn test_respose_from_slice() {
    use crate::de::from_slice;
    let bytes = vec![
        0x81, 0x0, 0x0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc,
        0xdd, 0xee, 0xff, 0x0, 0x0, 0x0, 0xc8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3,
        0x0, 0x0, 0x0, 0x0, 0x4, 0x68, 0x6f, 0x73, 0x74, 0x3, 0x0, 0x0, 0x0, 0x0, 0x10, 0x2f, 0x31,
        0x32, 0x37, 0x2e, 0x30, 0x2e, 0x30, 0x2e, 0x31, 0x3a, 0x31, 0x32, 0x33, 0x34, 0x35, 0x0,
        0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x1, 0x1, 0x0, 0x0, 0x0, 0x0, 0x1d,
    ];

    let resp = from_slice(&bytes).unwrap();

    let expected = Response::builder()
        .version(0x81)
        .request_id(Some(Uuid::from_bytes([
            0x0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ])))
        .status_code(200)
        .status_message(Some("".to_owned()))
        .status_attribute(HashMap::from([("host".into(), "/127.0.0.1:12345".into())]))
        .result_meta(HashMap::new())
        .result_data(vec![29_i32].into())
        .build();

    assert_eq!(expected, resp)
}

#[test]
fn test_respose_with_t_nested() {
    let bytes = vec![
        0x81, 0x0, 0x0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc,
        0xdd, 0xee, 0xff, 0x0, 0x0, 0x0, 0xc8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3,
        0x0, 0x0, 0x0, 0x0, 0x4, 0x68, 0x6f, 0x73, 0x74, 0x3, 0x0, 0x0, 0x0, 0x0, 0x10, 0x2f, 0x31,
        0x32, 0x37, 0x2e, 0x30, 0x2e, 0x30, 0x2e, 0x31, 0x3a, 0x31, 0x32, 0x33, 0x34, 0x35, 0x0,
        0x0, 0x0, 0x0, 0x20, 0x0, 0x03, 0x0, 0x0, 0x0, 0x0, 0x2, b'i', b'd',
    ];

    let resp = from_slice(&bytes).unwrap();

    let expected = Response::builder()
        .version(0x81)
        .request_id(Some(Uuid::from_bytes([
            0x0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ])))
        .status_code(200)
        .status_message(Some("".to_owned()))
        .status_attribute(HashMap::from([("host".into(), "/127.0.0.1:12345".into())]))
        .result_meta(HashMap::new())
        .result_data(T::Id.into())
        .build();

    assert_eq!(expected, resp);
}

#[test]
fn print_msg() {
    let mut args = HashMap::new();

    args.insert(
        MapKeys::String("gremlin".to_string()),
        GraphBinary::String("g.V(x).values('age')".to_string()),
    );
    args.insert(
        MapKeys::String("language".to_string()),
        GraphBinary::String("gremlin-groovy".to_string()),
    );

    let mut bindings = HashMap::new();
    bindings.insert(
        MapKeys::String("x".to_string()),
        GraphBinary::String("1".to_string()),
    );

    args.insert(
        MapKeys::String("bindings".to_string()),
        GraphBinary::Map(bindings),
    );

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
    req.write_gb_bytes(&mut buf, "application/vnd.graphbinary-v1.0")
        .unwrap();
    println!("printing: {:?}", buf);
}

#[test]
fn print_msg1() {
    let mut args = HashMap::new();

    args.insert(
        MapKeys::String("gremlin".to_string()),
        GraphBinary::ByteCode(ByteCode {
            steps: vec![
                Step {
                    name: "inject".to_string(),
                    values: vec![1.into(), 2.into(), 3.into(), 4.into(), 5.into()],
                },
                Step {
                    name: "fold".to_string(),
                    values: vec![
                        0.into(),
                        Lambda {
                            language: "groovy".to_string(),
                            script: "{ a,b -> a+b }".to_string(),
                            arguments_length: 2,
                        }
                        .into(),
                    ],
                },
            ],
            sources: vec![],
        }),
    );
    args.insert(
        MapKeys::String("aliases".to_string()),
        GraphBinary::Map(HashMap::from([("g".into(), "g".into())])),
    );

    // args.insert(
    //     MapKeys::String("language".to_string()),
    //     GraphBinary::String("gremlin-groovy".to_string()),
    // );

    // let mut bindings = HashMap::new();
    // bindings.insert(
    //     MapKeys::String("x".to_string()),
    //     GraphBinary::String("1".to_string()),
    // );

    // args.insert(
    //     MapKeys::String("bindings".to_string()),
    //     GraphBinary::Map(bindings),
    // );

    let req = Request {
        version: 0x81,
        request_id: uuid::Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]),
        op: "bytecode".to_owned(),
        processor: "traversal".to_owned(),
        args,
    };

    let mut buf: Vec<u8> = vec![];
    req.write_gb_bytes(&mut buf, "application/vnd.graphbinary-v1.0")
        .unwrap();
    println!("printing: {:?}", buf);
}

#[test]
fn test_respose_from_slice1() {
    use crate::de::from_slice;
    let bytes = vec![
        0x81, 0x0, 0x0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc,
        0xdd, 0xee, 0xff, 0x0, 0x0, 0x0, 0xc8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3,
        0x0, 0x0, 0x0, 0x0, 0x4, 0x68, 0x6f, 0x73, 0x74, 0x3, 0x0, 0x0, 0x0, 0x0, 0x11, 0x2f, 0x31,
        0x37, 0x32, 0x2e, 0x32, 0x31, 0x2e, 0x30, 0x2e, 0x31, 0x3a, 0x34, 0x36, 0x37, 0x37, 0x36,
        0x0, 0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x4, 0x21, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x1, 0x11, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x6,
        0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x1, 0x21, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x1, 0x11, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x6,
        0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x1, 0x21, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x1, 0x11, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x4, 0x0, 0x0, 0x0, 0x6,
        0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x1, 0x21, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x1, 0x11, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x6, 0x0, 0x0, 0x0, 0x6,
        0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x1,
    ];

    let resp: Response = from_slice(&bytes).unwrap();

    // let expected = Response::builder()
    //     .version(0x81)
    //     .request_id(Some(Uuid::from_bytes([
    //         0x0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
    //         0xee, 0xff,
    //     ])))
    //     .status_code(200)
    //     .status_message(Some("".to_owned()))
    //     .status_attribute(HashMap::from([("host".into(), "/127.0.0.1:12345".into())]))
    //     .result_meta(HashMap::new())
    //     .result_data(vec![29_i32].into())
    //     .build();

    print!("{:?}", resp);
    // assert_eq!(expected, resp)
}

#[test]
fn test() {
    let req = Request::builder()
        .session("aklshdJBASFKABFHuh1KJBJKlkjA")
        .request_id(Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]))
        .eval()
        .alias("g", "social")
        .alias("t", "test")
        .gremlin("social.V(x).values('age')")
        .bindings(HashMap::from([("x".into(), 1_i32.into())]))
        .build();

    let mut args: HashMap<MapKeys, GraphBinary> = HashMap::<MapKeys, GraphBinary>::from([
        ("session".into(), "aklshdJBASFKABFHuh1KJBJKlkjA".into()),
        ("language".into(), "gremlin-groovy".to_string().into()),
        ("gremlin".into(), "social.V(x).values('age')".into()),
    ]);

    args.insert(
        "bindings".into(),
        HashMap::<String, GraphBinary>::from([("x".into(), 1_i32.into())]).into(),
    );
    args.insert(
        "aliases".into(),
        HashMap::<String, GraphBinary>::from([
            ("g".into(), "social".into()),
            ("t".into(), "test".into()),
        ])
        .into(),
    );

    assert_eq!(
        Request {
            version: 0x81,
            request_id: Uuid::from_bytes([
                0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
                0xee, 0xff
            ]),
            op: "eval".to_string(),
            processor: "session".to_string(),
            args
        },
        req
    )
}
