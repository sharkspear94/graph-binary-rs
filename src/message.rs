use std::collections::HashMap;

use crate::graph_binary::{Encode, GraphBinary, MapKeys, VALUE_PRESENT};
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
        0x81, // Graphbinary version
        0x00,
        0x11,
        0x22,
        0x33,
        0x44,
        0x55,
        0x66,
        0x77,
        0x88,
        0x99,
        0xaa,
        0xbb,
        0xcc,
        0xdd,
        0xee,
        0xff, // Uuid 00112233-4455-6677-8899-aabbccddeeff
        0x00,
        0x00,
        0x00,
        0x04,
        b'e',
        b'v',
        b'a',
        b'l', // op = "eval"
        0x00,
        0x00,
        0x00,
        0x00, // processor empty string
        0x00,
        0x00,
        0x00,
        0x03, // args map length
        CORE_TYPE_STRING,
        VALUE_PRESENT, //string key
        0x00,
        0x00,
        0x00,
        0x07, //string length
        b'g',
        b'r',
        b'e',
        b'm',
        b'l',
        b'i',
        b'n', //gremlin
        CORE_TYPE_STRING,
        VALUE_PRESENT, //string value
        0x00,
        0x00,
        0x00,
        0x06, //string lenth
        b'g',
        b'.',
        b'V',
        b'(',
        b'x',
        b')', // g.V(x)
        CORE_TYPE_STRING,
        VALUE_PRESENT, //string key
        0x00,
        0x00,
        0x00,
        0x08, //string length
        b'l',
        b'a',
        b'n',
        b'g',
        b'u',
        b'a',
        b'g',
        b'e', //language
        CORE_TYPE_STRING,
        VALUE_PRESENT, //string value
        0x00,
        0x00,
        0x00,
        0x0e, //string length
        b'g',
        b'r',
        b'e',
        b'm',
        b'l',
        b'i',
        b'n',
        b'-',
        b'g',
        b'r',
        b'o',
        b'o',
        b'v',
        b'y', //gremlin-groovy
        CORE_TYPE_STRING,
        VALUE_PRESENT, // string key bindings
        0x00,
        0x00,
        0x00,
        0x08,
        b'b',
        b'i',
        b'n',
        b'd',
        b'i',
        b'n',
        b'g',
        b's',
        CORE_TYPE_BINDING,
        VALUE_PRESENT, //binding
        0x00,
        0x00,
        0x00,
        0x01, //string lenth of binding key
        b'x', //key
        CORE_TYPE_INT,
        VALUE_PRESENT, //fq int with value 1
        0x00,
        0x00,
        0x00,
        0x01,
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

struct Response {
    version: u8,
    request_id: Option<uuid::Uuid>,
    status_code: i32,
    status_message: Option<String>,
    status_attribute: Map,
    result_meta: Map,
    result_data: GraphBinary,
}

#[test]
fn print_msg() {
    let mut args = HashMap::new();

    args.insert(
        MapKeys::String("gremlin".to_string()),
        GraphBinary::String("g.V(x).outE('created')".to_string()),
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
}
