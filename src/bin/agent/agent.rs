
use std::{sync::Arc, thread::sleep, time::Duration};

#[path = "../../common/mod.rs"]
mod common;
use common::{*, 
            networkinterface::*,
            queue::*,
            protocol::*,
            parser::*,
            command_types::*,
            };


fn main() {

    let parser = Parser::new();
    let interface = NetworkInterface::new(("127.0.0.1".to_string(), "6960".to_string()), ("127.0.0.1".to_string(), "6900".to_string()));

    let inbound_queue = Arc::new(Queue::<Packet>::new());
    let outbound_queue = Arc::new(Queue::<Packet>::new());

    listen_thread(&interface, &inbound_queue);
    send_thread(&interface, &outbound_queue);

    outbound_queue.push(parser.new_guid_post("127.0.0.1,6960".to_string()));


    loop {
        sleep(Duration::new(0,50000));
        if inbound_queue.is_empty() {
            continue;
        }
        else {
            let inbound_packet = parser.ingest_payload(inbound_queue.pop());
            let destination_guid = inbound_packet.0;                        
            let payload = inbound_packet.2;

            let command:CommandEnum = Parser::deserialize_payload(payload);
            
            let response = command.execute();
            outbound_queue.push(parser.new_response(destination_guid, response));
        }
    }


    
}