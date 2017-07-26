extern crate zmq;
extern crate lmcp;

use std::str;

#[derive(Debug)]
struct MessageAttributes {
    content_type: String,
    descriptor: String,
    source_group: String,
    source_entity_id: String,
    source_service_id: String,
}

fn main() {
    let ctx = zmq::Context::new();
    let sub = ctx.socket(zmq::SUB).unwrap();
    assert!(sub.connect("tcp://localhost:5560").is_ok());
    sub.set_subscribe(&[]).unwrap();
    loop {
        let msg = sub.recv_bytes(0).unwrap();
        let mut parts = msg.splitn(3, |&b| b == 36); // 36 == $
        let address = parts.next().unwrap();
        let mut atts_iter = parts.next().unwrap().split(|&b| b == 124); // 124 == |
        let attributes = MessageAttributes {
            content_type: String::from(str::from_utf8(atts_iter.next().unwrap()).unwrap()),
            descriptor: String::from(str::from_utf8(atts_iter.next().unwrap()).unwrap()),
            source_group: String::from(str::from_utf8(atts_iter.next().unwrap()).unwrap()),
            source_entity_id: String::from(str::from_utf8(atts_iter.next().unwrap()).unwrap()),
            source_service_id: String::from(str::from_utf8(atts_iter.next().unwrap()).unwrap()),
        };
        let payload = parts.next().unwrap();
        println!("Address: {}", str::from_utf8(address).unwrap());
        println!("Attributes: {:?}", attributes);
        match lmcp::lmcp_msg_deser(&payload) {
            Ok(v) => {
                println!("Payload deserialized successfully");
//                println!("Payload: {:?}", v);
            }
            Err(()) => {
                println!("Error deserializing payload: {:?}", payload);
                panic!("Full message: {:?}", msg);
            },
        }
    }
}
