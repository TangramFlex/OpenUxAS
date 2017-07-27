use std::result;
use std::str;

use lmcp::*;
use lmcp::avtas::lmcp::{LmcpSer};
use zmq;

#[derive(Debug)]
struct MessageAttributes {
    content_type: String,
    descriptor: String,
    source_group: String,
    source_entity_id: String,
    source_service_id: String,
}

impl MessageAttributes {
    fn to_uxas_header(&self) -> String {
        format!("{}|{}|{}|{}|{}",
                self.content_type,
                self.descriptor,
                self.source_group,
                self.source_entity_id,
                self.source_service_id,
                )
    }
}

pub struct Client {
    pub entity_id: String,
    pub service_id: String,
    sub_socket: zmq::Socket,
    push_socket: zmq::Socket,
}

#[derive(Debug)]
pub enum Error {
    Zmq(zmq::Error),
    Lmcp,
}

pub type Result<T> = result::Result<T, Error>;

impl Client {
    pub fn new(ctx: zmq::Context, entity_id: &str, service_id: &str) -> Result<Self> {
        let sub_socket = ctx.socket(zmq::SUB).map_err(Error::Zmq)?;
        let push_socket = ctx.socket(zmq::PUSH).map_err(Error::Zmq)?;
        Ok(Client {
            entity_id: String::from(entity_id),
            service_id: String::from(service_id),
            sub_socket: sub_socket,
            push_socket: push_socket,
        })
    }

    pub fn connect(&self, endpoint: &str, sub_port: u16, push_port: u16) -> Result<()> {
        self.sub_socket.connect(&format!("{}:{}", endpoint, sub_port)).map_err(Error::Zmq)?;
        self.push_socket.connect(&format!("{}:{}", endpoint, push_port)).map_err(Error::Zmq)
    }

    pub fn add_subscription(&self, topic: &[u8]) -> Result<()> {
        self.sub_socket.set_subscribe(topic).map_err(Error::Zmq)
    }

    pub fn broadcast(&self, obj: &LmcpType) -> Result<()> {
        let mut msg = lmcp_msg_subscription(obj).as_bytes().to_vec();
        msg.push(36); // $
        let attrs = MessageAttributes {
            content_type: String::from("lmcp"),
            descriptor: String::from(lmcp_msg_subscription(obj)),
            source_group: String::from(""),
            source_entity_id: self.entity_id.clone(),
            source_service_id: self.service_id.clone(),
        };
        msg.append(&mut attrs.to_uxas_header().into_bytes());
        msg.push(36); // $
        let mut payload: Vec<u8> = vec![0; lmcp_msg_size(obj)];
        lmcp_msg_ser(obj, &mut payload).map_err(|_| Error::Lmcp)?;
        msg.append(&mut payload);
        println!("sending attrs: {:?}", attrs);
        self.push_socket.send(&msg, 0).map_err(Error::Zmq)
    }

    pub fn receive(&self) -> Result<LmcpType> {
        let msg = self.sub_socket.recv_bytes(0).map_err(Error::Zmq)?;

        let mut parts = msg.splitn(3, |&b| b == 36); // 36 == $
        let _address = parts.next().ok_or(Error::Lmcp)?;
        let mut atts_iter = parts.next().ok_or(Error::Lmcp)?.split(|&b| b == 124); // 124 == |
        let _attributes = MessageAttributes {
            content_type: String::from(str::from_utf8(atts_iter.next().ok_or(Error::Lmcp)?).map_err(|_| Error::Lmcp)?),
            descriptor: String::from(str::from_utf8(atts_iter.next().ok_or(Error::Lmcp)?).map_err(|_| Error::Lmcp)?),
            source_group: String::from(str::from_utf8(atts_iter.next().ok_or(Error::Lmcp)?).map_err(|_| Error::Lmcp)?),
            source_entity_id: String::from(str::from_utf8(atts_iter.next().ok_or(Error::Lmcp)?).map_err(|_| Error::Lmcp)?),
            source_service_id: String::from(str::from_utf8(atts_iter.next().ok_or(Error::Lmcp)?).map_err(|_| Error::Lmcp)?),
        };
        let payload = parts.next().ok_or(Error::Lmcp)?;
        lmcp_msg_deser(&payload).map_err(|_| Error::Lmcp)?.ok_or(Error::Lmcp)
    }
}
