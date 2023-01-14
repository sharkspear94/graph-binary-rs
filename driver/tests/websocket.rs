use std::{collections::HashMap, thread, vec};

use driver::{
    message::{Request, Response},
    process::{graph_traversal_source::GraphTraversalSource, traversal::__},
    *,
};
use gremlin_types::{
    binary::Decode,
    structure::{
        bytecode::Bytecode,
        enums::{Order, TextP, P, T},
        lambda::Lambda,
    },
};
use serde::Deserialize;
use websocket::{dataframe::Opcode, ws::Message, *};

#[test]
fn nested_coalecse() {
    let mut g = GraphTraversalSource::<()>::new();
    let bc = g
        .v(())
        .coalesce([__.out_e("created"), __.out_e("knows")])
        .in_v()
        .path()
        .by("name")
        .by(T::Label);
    let req = Request::builder()
        .request_id(uuid::Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]))
        .bytecode()
        .gremlin(bc.bytecode)
        .build();

    let client = ClientBuilder::new("ws://localhost:8182/gremlin")
        .unwrap()
        .connect_insecure()
        .unwrap();
    let mut buf: Vec<u8> = vec![];
    req.write_gb_bytes(&mut buf, "application/vnd.graphbinary-v1.0")
        .unwrap();
    // println!("{buf:?}");
    let msg = websocket::OwnedMessage::Binary(buf);

    let (mut resv, mut sender) = client.split().unwrap();

    // let o_msg = client.recv_dataframe().unwrap();
    let handle = thread::spawn(move || {
        let msg = resv.recv_dataframe().unwrap();
        let resp = Response::decode(&mut &msg.data[..]);
        assert!(resp.is_ok());
        println!("{:?}", resp);
    });

    sender.send_message(&msg).unwrap();

    // let recv: Response = from_slice(&o_msg.data).unwrap();
    handle.join().unwrap();
    // print!("{:?}", recv);
}

#[test]
fn test() {
    let mut g = GraphTraversalSource::<()>::new();
    let bc = g
        .v(())
        .has_label("person")
        .both_e(())
        .order(())
        .by(("weight", Order::Desc));

    let req = Request::builder()
        .request_id(uuid::Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]))
        .bytecode()
        .gremlin(bc.bytecode)
        .build();

    let mut client = ClientBuilder::new("ws://localhost:8182/gremlin")
        .unwrap()
        .connect(None)
        .unwrap();
    let mut buf: Vec<u8> = vec![];
    req.write_gb_bytes(&mut buf, "application/vnd.graphbinary-v1.0")
        .unwrap();
    // let buf = to_bytes(req).unwrap();
    println!("{buf:?}");
    let msg = websocket::OwnedMessage::Binary(buf);

    client.send_message(&msg).unwrap();
    let o_msg = client.recv_dataframe().unwrap();

    let recv = Response::decode(&mut &o_msg.data[..]).unwrap();
    let x = recv.unwind_traverser();
    print!("{:?}", recv);
    print!("\n\n");
    println!("{:?}", x);
}

#[test]
fn test2() {
    let mut bc = Bytecode::new();
    bc.push_new_step("V", vec![1.into()]);
    bc.push_new_step("outE", vec!["knows".into()]);
    bc.push_new_step("elementMap", vec![]);

    let req = Request::builder()
        .request_id(uuid::Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]))
        .bytecode()
        .gremlin(bc)
        .build();

    let mut client = ClientBuilder::new("ws://localhost:8182/gremlin")
        .unwrap()
        .connect(None)
        .unwrap();
    let mut buf: Vec<u8> = vec![];
    req.write_gb_bytes(&mut buf, "application/vnd.graphbinary-v1.0")
        .unwrap();
    // let buf = to_bytes(req).unwrap();
    println!("{buf:?}");
    let msg = websocket::OwnedMessage::Binary(buf);

    client.send_message(&msg).unwrap();
    let o_msg = client.recv_dataframe().unwrap();

    let recv: Response = Response::decode(&mut &o_msg.data[..]).unwrap();
    print!("{:?}", recv);
    #[derive(Debug, Deserialize)]
    struct Test {
        id: i64,
        label: String,
        age: i32,
        name: String,
    }

    // let test = from_graph_binary::<Vec<Test>>(recv.result_data()).unwrap();

    // print!("{:?}", test);
}

#[test]
fn test_textP() {
    let mut bc = Bytecode::new();
    bc.push_new_step("V", vec![1.into()]);
    bc.push_new_step("has", vec!["name".into(), TextP::starting_with("m").into()]);
    bc.push_new_step("values", vec!["name".into()]);

    let req = Request::builder()
        .request_id(uuid::Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]))
        .bytecode()
        .gremlin(bc)
        .build();

    let mut client = ClientBuilder::new("ws://localhost:8182/gremlin")
        .unwrap()
        .connect(None)
        .unwrap();
    let mut buf: Vec<u8> = vec![];
    req.write_gb_bytes(&mut buf, "application/vnd.graphbinary-v1.0")
        .unwrap();
    // let buf = to_bytes(req).unwrap();
    println!("{buf:?}");
    let msg = websocket::OwnedMessage::Binary(buf);

    client.send_message(&msg).unwrap();
    let o_msg = client.recv_dataframe().unwrap();

    let recv: Response = Response::decode(&mut &o_msg.data[..]).unwrap();
    print!("{:?}", recv);
}

#[test]
fn test3() {
    let req = Request::builder()
        .request_id(uuid::Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]))
        .eval()
        .gremlin("g.V(1).elementMap('age','name').next()")
        .build();

    let mut client = ClientBuilder::new("ws://localhost:8182/gremlin")
        .unwrap()
        .connect(None)
        .unwrap();
    let mut buf: Vec<u8> = vec![];
    req.write_gb_bytes(&mut buf, "application/vnd.graphbinary-v1.0")
        .unwrap();
    // let buf = to_bytes(req).unwrap();
    println!("{buf:?}");
    let msg = websocket::OwnedMessage::Binary(buf);

    client.send_message(&msg).unwrap();
    let o_msg = client.recv_dataframe().unwrap();

    let recv: Response = Response::decode(&mut &o_msg.data[..]).unwrap();
    print!("{:?}", recv);
    #[derive(Debug, Deserialize)]
    struct Test {
        id: i64,
        label: String,
        age: i32,
        name: String,
    }

    // let test = from_graph_binary::<Vec<Test>>(recv.result_data()).unwrap();

    // print!("{:?}", test);
}

#[test]
fn test_addE() {
    let mut bc = Bytecode::new();
    let mut bc1 = Bytecode::new();
    let mut bc2 = Bytecode::new();
    bc.push_new_step("addE", vec!["testLabel".into()]);
    bc1.push_new_step("V", vec![1.into()]);
    bc.push_new_step("from", vec![bc1.into()]);
    bc2.push_new_step("V", vec![]);
    bc2.push_new_step("has", vec!["name".into(), "josh".into()]);
    bc.push_new_step("to", vec![bc2.into()]);

    let req = Request::builder()
        .request_id(uuid::Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]))
        .bytecode()
        .gremlin(bc)
        .build();

    let mut client = ClientBuilder::new("ws://localhost:8182/gremlin")
        .unwrap()
        .connect(None)
        .unwrap();
    let mut buf: Vec<u8> = vec![];
    req.write_gb_bytes(&mut buf, "application/vnd.graphbinary-v1.0")
        .unwrap();
    // let buf = to_bytes(req).unwrap();
    println!("{buf:?}");
    let msg = websocket::OwnedMessage::Binary(buf);

    client.send_message(&msg).unwrap();
    let o_msg = client.recv_dataframe().unwrap();

    let recv: Response = Response::decode(&mut &o_msg.data[..]).unwrap();
    print!("{:?}", recv);
}

#[test]
fn test_lambda() {
    let mut bc = Bytecode::new();

    bc.push_new_step("inject", vec![1.into(), 2.into()]);
    // bc.push_new_step(
    //     "addV",
    //     vec![HashMap::from([("name", "felix".into()), ("age", GraphBinary::from(28))]).into()],
    // );
    bc.push_new_step(
        "fold",
        vec![
            1.into(),
            Lambda {
                language: "gremlin-groovy".to_string(),
                script: "{ it.get().toString() }".to_string(),
                arguments_length: 0,
            }
            .into(),
        ],
    );
    // bc.push_new_step("from", vec![bc1.into()]);
    // bc2.push_new_step("V", vec![]);
    // bc2.push_new_step("has", vec!["name".into(), "josh".into()]);
    // bc.push_new_step("to", vec![bc2.into()]);

    let req = Request::builder()
        .request_id(uuid::Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]))
        .bytecode()
        .gremlin(bc)
        .build();

    let mut client = ClientBuilder::new("ws://localhost:8182/gremlin")
        .unwrap()
        .connect(None)
        .unwrap();
    let mut buf: Vec<u8> = vec![];
    req.write_gb_bytes(&mut buf, "application/vnd.graphbinary-v1.0")
        .unwrap();
    // let buf = to_bytes(req).unwrap();
    println!("{buf:?}");
    let msg = websocket::OwnedMessage::Binary(buf);

    client.send_message(&msg).unwrap();
    let o_msg = client.recv_dataframe().unwrap();

    let recv: Response = Response::decode(&mut &o_msg.data[..]).unwrap();
    print!("{:?}", recv);
}

#[test]
fn test_sack() {
    let mut g = GraphTraversalSource::<()>::new();
    let bc = g
        // .with_sack(Lambda::new("{[:]}{it.clone()}"))
        .with_sack(1)
        .v(())
        // .out(())
        // .out(())
        // .sack(Lambda::new(
        //     "{m,v -> m[v.value('name')] = v.value('lang'); m}",
        // ))
        .sack(());

    let req = Request::builder()
        .request_id(uuid::Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]))
        .bytecode()
        .gremlin(bc.bytecode)
        .build();

    let mut client = ClientBuilder::new("ws://localhost:8182/gremlin")
        .unwrap()
        .connect(None)
        .unwrap();
    let mut buf: Vec<u8> = vec![];
    req.write_gb_bytes(&mut buf, "application/vnd.graphbinary-v1.0")
        .unwrap();
    // let buf = to_bytes(req).unwrap();
    println!("{buf:?}");
    // let msg = websocket::OwnedMessage::Binary(buf);
    let msg = websocket::Message::binary(buf);

    client.send_message(&msg).unwrap();
    let o_msg = client.recv_dataframe().unwrap();

    let recv: Response = Response::decode(&mut &o_msg.data[..]).unwrap();
    let x = recv.unwind_traverser();

    print!("{:?}", x);
}

#[test]
fn test_profile() {
    let mut g = GraphTraversalSource::<()>::new();
    let bc = g
        .v(())
        .out(())
        .in_(())
        .where_(__.out(()).has_label("software"))
        .profile(());

    let req = Request::builder()
        .request_id(uuid::Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]))
        .bytecode()
        .gremlin(bc.bytecode)
        .build();

    let mut client = ClientBuilder::new("ws://localhost:8182/gremlin")
        .unwrap()
        .connect(None)
        .unwrap();
    let mut buf: Vec<u8> = vec![];
    req.write_gb_bytes(&mut buf, "application/vnd.graphbinary-v1.0")
        .unwrap();
    // let buf = to_bytes(req).unwrap();
    println!("{buf:?}");
    // let msg = websocket::OwnedMessage::Binary(buf);
    let msg = websocket::Message::binary(buf);

    client.send_message(&msg).unwrap();
    let o_msg = client.recv_dataframe().unwrap();

    let recv: Response = Response::decode(&mut &o_msg.data[..]).unwrap();
    let x = recv.unwind_traverser();

    print!("{:?}", x);
}

#[test]
fn test_has() {
    let mut g = GraphTraversalSource::<()>::new();
    let bc = g.v(()).has(("name", "marko"));

    let req = Request::builder()
        .request_id(uuid::Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]))
        .bytecode()
        .gremlin(bc.bytecode)
        .build();

    let mut client = ClientBuilder::new("ws://localhost:8182/gremlin")
        .unwrap()
        .connect(None)
        .unwrap();
    let mut buf: Vec<u8> = vec![];
    req.write_gb_bytes(&mut buf, "application/vnd.graphbinary-v1.0")
        .unwrap();
    // let buf = to_bytes(req).unwrap();
    println!("{buf:?}");
    // let msg = websocket::OwnedMessage::Binary(buf);
    let msg = websocket::Message::binary(buf);

    client.send_message(&msg).unwrap();
    let o_msg = client.recv_dataframe().unwrap();

    let recv: Response = Response::decode(&mut &o_msg.data[..]).unwrap();
    let x = recv.unwind_traverser();

    print!("{:?}", x);
}

#[test]
fn test_tree() {
    let mut g = GraphTraversalSource::<()>::new();
    let bc = g.v(()).out(()).out(()).tree(());

    let req = Request::builder()
        .request_id(uuid::Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]))
        .bytecode()
        .gremlin(bc.bytecode)
        .build();

    let mut client = ClientBuilder::new("ws://localhost:8182/gremlin")
        .unwrap()
        .connect(None)
        .unwrap();
    let mut buf: Vec<u8> = vec![];
    req.write_gb_bytes(&mut buf, "application/vnd.graphbinary-v1.0")
        .unwrap();
    println!("{buf:?}");
    let msg = websocket::Message::binary(buf);

    client.send_message(&msg).unwrap();
    let o_msg = client.recv_dataframe().unwrap();
    println!("{:?}", o_msg);
    // let recv: Response = from_slice(&o_msg.data).unwrap();
    // let x = recv.unwind_traverser();

    // print!("{:?}", x);
}

#[test]
fn test_iter() {
    let mut g = GraphTraversalSource::<()>::new();
    let mut bc = g;
    // .subgraph("sub").cap("sub", ());
    // .order(()).by(("age", Order::Desc));
    let mut a = Bytecode::new();
    a.push_new_step("io", vec!["test.json".into()]);
    a.push_new_step("write", vec![]);
    // let mut a = bc.bytecode().clone();
    // a.push_new_step("a", vec![]);
    println!("{}", a);
    let req = Request::builder()
        .request_id(uuid::Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]))
        .bytecode()
        .gremlin(a)
        .build();

    let mut client = ClientBuilder::new("ws://localhost:8182/gremlin")
        .unwrap()
        .connect(None)
        .unwrap();
    let mut buf: Vec<u8> = vec![];
    req.write_gb_bytes(&mut buf, "application/vnd.graphbinary-v1.0")
        .unwrap();
    println!("{buf:?}");
    let msg = websocket::Message::binary(buf);

    client.send_message(&msg).unwrap();
    let o_msg = client.recv_dataframe().unwrap();

    // println!("{:?}", o_msg);
    let recv: Response = Response::decode(&mut &o_msg.data[..]).unwrap();
    let mut status = o_msg.opcode;
    let mut v = vec![recv];
    println!("{:?}", v);
    while status == Opcode::Continuation {
        let o_msg = client.recv_dataframe().unwrap();
        let recv: Response = Response::decode(&mut &o_msg.data[..]).unwrap();
        status = o_msg.opcode;
        v.push(recv)
    }
    println!("{:?}", v);
    // let x = recv.unwind_traverser();

    // print!("{:?}", o_msg);
}
