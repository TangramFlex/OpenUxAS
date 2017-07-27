extern crate standalone_planbuilder;
extern crate zmq;

use standalone_planbuilder::client::{Client};
use standalone_planbuilder::plan_builder::{PlanBuilder};

use std::thread;

fn main() {
    let ctx = zmq::Context::new();
    let client = Client::new(ctx, "101", "1337").unwrap();
    client.connect("tcp://localhost", 5560, 5561).unwrap();
    let pb: thread::JoinHandle<_> = PlanBuilder::run(client, 0.0, 45.323, -120.9645);
    pb.join().unwrap();
}
