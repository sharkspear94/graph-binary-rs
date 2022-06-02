use std::collections::HashMap;

use serde::de::Visitor;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::EncodeError;
use crate::graph_binary::{Decode, Encode, GraphBinary, MapKeys};

struct Request {
    version: u8,
    request_id: uuid::Uuid,
    op: String,
    processor: String,
    args: HashMap<MapKeys, GraphBinary>,
}

impl Request {
    fn write_gb_respons_bytes<W: std::io::Write>(
        &self,
        writer: &mut W,
        mime_type: &str,
    ) -> Result<(), EncodeError> {
        writer.write_all(&[mime_type.len() as u8])?;
        writer.write_all(mime_type.as_bytes())?;
        self.write_full_qualified_bytes(writer)
    }
}

impl Default for Request {
    fn default() -> Self {
        Self {
            version: 0x81,
            request_id: Uuid::new_v4(),
            op: "eval".to_owned(),
            processor: String::default(),
            args: HashMap::from([("language".into(), "gremlin-groovy".into())]),
        }
    }
}

impl Encode for Request {
    fn type_code() -> u8 {
        unimplemented!("not supported for Request")
    }

    fn write_patial_bytes<W: std::io::Write>(
        &self,
        _writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        unimplemented!("")
    }
    fn write_full_qualified_bytes<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.version.write_patial_bytes(writer)?;
        self.request_id.write_patial_bytes(writer)?;
        self.op.write_patial_bytes(writer)?;
        self.processor.write_patial_bytes(writer)?;
        self.args.write_patial_bytes(writer)
    }
}

struct RequestBuilder(Request);

impl Request {
    fn builder() -> RequestBuilder {
        RequestBuilder(Request::default())
    }
}

impl RequestBuilder {
    fn version(mut self, version: u8) -> Self {
        self.0.version = version;
        self
    }
    fn request_id(mut self, request_id: Uuid) -> Self {
        self.0.request_id = request_id;
        self
    }
    fn op(mut self, op: &str) -> Self {
        self.0.op = op.to_owned();
        self
    }
    fn processor(mut self, processor: &str) -> Self {
        self.0.processor = processor.to_owned();
        self
    }
    fn language(mut self, language: &str) -> Self {
        self.0.args.insert("language".into(), language.into());
        self
    }
    fn script(mut self, script_lang: &str, script: &str) -> Self {
        self.0.args.insert(script_lang.into(), script.into());
        self
    }
    fn bindings(mut self, bindings: HashMap<MapKeys, GraphBinary>) -> Self {
        self.0.args.insert("bindings".into(), bindings.into());
        self
    }
    fn build(self) -> Request {
        self.0
    }
}

#[derive(Debug, PartialEq)]
struct Response {
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
}

#[derive(Debug)]
struct ResponseBuilder {
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
        Response::fully_self_decode(&mut v)
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

    fn fully_self_decode<R: std::io::Read>(
        reader: &mut R,
    ) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let version = u8::partial_decode(reader)?;
        let uuid = Uuid::partial_nullable_decode(reader)?;
        let status_code = i32::partial_decode(reader)?;
        let status_message = String::partial_nullable_decode(reader)?;
        let status_attributes = HashMap::<MapKeys, GraphBinary>::partial_decode(reader)?;
        let result_meta = HashMap::<MapKeys, GraphBinary>::partial_decode(reader)?;
        let result_data = GraphBinary::fully_self_decode(reader)?;

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

    fn partial_count_bytes(_bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
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

    req.write_full_qualified_bytes(&mut buf).unwrap();
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

    req.write_gb_respons_bytes(&mut buf, "application/vnd.graphbinary-v1.0")
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

    let resp = Response::fully_self_decode(&mut &*bytes).unwrap();

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
    req.write_gb_respons_bytes(&mut buf, "application/vnd.graphbinary-v1.0")
        .unwrap();
    println!("printing: {:?}", buf);
}
