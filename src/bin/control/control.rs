#[path = "../../common/mod.rs"]
mod common;
use std::{collections::HashMap, sync::Arc, thread::sleep, time::Duration};

use common::{
    networkinterface::*,
    queue::*,
    protocol::*,
};

mod guid_handler;
use guid_handler::*;

fn main(){
    let debug_source = ("127.0.0.1".to_string(), 6900.to_string());
    let debug_destination = ("127.0.0.1".to_string(), 5959.to_string());
    let inbound_interface = NetworkInterface::new(debug_source.clone(), debug_destination.clone());

    let mut outbound_interface = NetworkInterface::new(debug_source, debug_destination);

    let inbound_queue = Arc::new(Queue::<Packet>::new());
    let outbound_queue = Arc::new(Queue::<Packet>::new());

    listen_thread(&inbound_interface, &inbound_queue);
    let mut control_handler = ControlHandler::new();

    loop {
        sleep(Duration::new(0,50000));
        if inbound_queue.is_empty() {
            continue;
        }
        else {
            // definitely clean this up, but should be a decent for testing
            let transit_packet = inbound_queue.pop();
            control_handler.analyze(transit_packet, &mut outbound_interface);
        }
    }
}

