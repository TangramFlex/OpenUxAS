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
    let mut msg = zmq::Message::new().unwrap();
    loop {
        sub.recv(&mut msg, 0).unwrap();
        // print!("[");
        // for b in msg.iter() {
        //     print!("{}({}), ", b, char::from(*b));
        // }
        // println!("]");

        let mut parts = msg.split(|&b| b == 36); // 36 == $
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
        let v = lmcp::lmcp_msg_deser(&payload);
        println!("Address: {}", str::from_utf8(address).unwrap());
        println!("Attributes: {:?}", attributes);
        println!("Payload: {:?}", v);
    }
}
