#[path = "../../common/mod.rs"]
mod common;
use std::{collections::HashMap, sync::Arc, thread::sleep, time::Duration};

use common::{
    netutils::*,
    queue::*,
    contract::*,
    guidhandler::*,
};


fn main(){
    let mut interface = NetworkInterface::new();
    let mut guid_handler = RelayGuidHandler::new(interface);
    let listener = guid_handler.interface.create_listener("127.0.0.1:6969".to_string());



    loop {
        sleep(Duration::new(5,50000));
        if guid_handler.interface.inbound_queue.is_empty(){
            println!("Inbound queue empty");
            continue;
        }
        else {
            println!("Guid handler analyzing");
            guid_handler.analyze(&guid_handler.interface.inbound_queue.pop());
            println!("NEW GUID TABLE!!!! {:?}", guid_handler);
        }
        match &guid_handler.interface.outbound_queue.is_empty(){
            &_False => {
                    println!("True match on the outbound queue my brother");
                    let rhost = guid_handler.interface.outbound_queue.peek().0.clone();
                    guid_handler.interface.create_stream(rhost.clone());
                    guid_handler.interface.join_stream(rhost.clone());
            }
            &_True => {continue;}
        }

}
}

