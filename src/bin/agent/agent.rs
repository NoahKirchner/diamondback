
use std::{sync::Arc, thread::sleep, time::Duration};

#[path = "../../common/mod.rs"]
mod common;

use common::parser::*;
use common::netutils::NetworkInterface;
use common::queue::*;

fn main() {

    let parser = Parser::new("test".to_string());
    let mut interface = NetworkInterface::new();
    
    interface.create_listener("127.0.0.1:6968".to_string());
    interface.create_stream("127.0.0.1:6969".to_string());

  // mfer we dealing with guid sync later you dig??  
//    outbound_queue.push(parser.new_guid_post("127.0.0.1,6960".to_string()));


    loop {
        sleep(Duration::new(5,50000));
        if interface.inbound_queue.is_empty() {
            continue;
        }
        else {
            let parse_response = parser.parse(interface.inbound_queue.pop());
            match parse_response {
                
                ParsedValue::ParsedResponse { result, source} => {println!("HUH??")},
                ParsedValue::ParsedCommand { response } => {interface.outbound_queue.push(("TESTFIXMEEE".to_string(),response.clone()))}
                ParsedValue::Other => todo!()
            }
        }
    }


    
}
