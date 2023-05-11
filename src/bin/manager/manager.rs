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
use common::queue::*;
use common::netutils::*;
use common::parser::*;
use common::commandutils::*;
use common::contract::*;
use common::guidhandler::*;

use std::{
    fs, io,
    sync::Arc,
    thread::{self, sleep},
    time::Duration,
};



fn main() {
    println!("Manager");
    let parser = Parser::new("mangin".to_string());
    let mut interface = NetworkInterface::new();


    let mut guid_map: Vec<String> = vec![];

    interface.create_stream("127.0.0.1:6969".to_string());
    let listener = interface.create_listener("127.0.0.1:6968".to_string());
    
    let post_contract = client_guid_post(listener.0, listener.1);
    interface.outbound_queue.push(("127.0.0.1:6969".to_string(), post_contract)); 

    //let pid = parser.build_contract("test".to_string(), ContractType::Command, Option::from("get_pid".to_string()), None, None);
        
    

    loop {
        println!("{:?}", interface);
        sleep(Duration::new(5, 50000));
        if interface.inbound_queue.is_empty(){
            println!("Quite empty yes yes");
            //interface.outbound_queue.push(pid.clone());
            continue;
        }
        else {
            println!("not empty");
            let parse_response = parser.parse(interface.inbound_queue.pop());
 
            println!("Matching responses {:?}", parse_response);
            match parse_response {
                ParsedValue::ParsedResponse {result, source} => {println!("{}|{}", result[0],result[1])},
                ParsedValue::ParsedCommand {response } => {println!("HUH??")},
                _ => {println!("{:?}", parse_response)},

            }
        }
    }

        /*
    loop {
        sleep(Duration::new(5, 50000));
        if interface.inbound_queue.is_empty() {
            continue;
        } else {
            let command = parser.ingest_response(inbound_queue.pop());
            println!("{:?}", command);

            //if command.1 == "guid_sync" {
              //  for item in command.2 {
                //    if item != parser.get_guid() && item.len() > 0 {
                  //      guid_map.push(item.to_string());
                    }
                }
            }
        }
       // if guid_map.is_empty() {
        //    continue;
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
 */
}
