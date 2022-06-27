use std::io::Read;

use websocket::{sync::Client, ClientBuilder, Message};

pub struct GClient {
    websocket_pool: (),
    options: ()
}

#[test]
fn test() {
    // let mut client = ClientBuilder::new("ws://localhost:8182/gremlin")
    //     .unwrap()
    //     .connect(None)
    //     .unwrap();

    // let msg = Message::text("asd");
    // let bin = Message::binary([1, 3, 51_u8, 12, 3].as_slice());
    // client.send_message(&msg);
}
