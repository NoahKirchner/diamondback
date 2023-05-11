use std::collections::HashMap;

use crate::common;
use common::{
    networkinterface::*,
    queue::*,
    protocol::*,
};








pub trait HandlerMethods {

    fn new()->Self;
    
    fn analyze(&mut self, packet:Packet, interface:&mut NetworkInterface);

}

pub struct ControlHandler {
    guid_map:HashMap<String,(String,String)>,
}

impl HandlerMethods for ControlHandler {
    fn new()->Self{
        Self {
            guid_map:HashMap::<String,(String,String)>::new()
        }
    }

    fn analyze(&mut self, packet:Packet, interface:&mut NetworkInterface){
        let message_type = packet.get_message_type();
        let guid_map = Self::get_guids(self);
        let source_guid = packet.get_source_guid()
        .expect("GUID request has no source GUID.");

                                                    


        if message_type == "guid_post".to_string(){
            let payload = &packet.get_response()[0];
            let raw_address = payload.rsplit_once(",")
                                                    .expect("Failed to split address.");

            let address = (raw_address.0.to_owned(), raw_address.1.to_owned());


            let guid = packet.get_source_guid()
                                            .expect("No guid_post source GUID.");

            Self::update_map(self, guid, address);
            println!("GUID Map Updated: {:?}", self.guid_map)
        }
        
        else if message_type == "guid_request".to_string(){
            let mut response_packet = Packet::new("guid_sync".to_string());

            response_packet.pack_destination(source_guid.clone());
            response_packet.pack_response(guid_map);
            
            Self::redirect_interface(self, source_guid.clone(), interface);
            interface.send(response_packet);
            println!("GUID sync to {:?}", source_guid);

        }
        else {
            let destination_guid = packet.get_destination_guid()
                                                    .expect("Generic packet lacks destination GUID");
            Self::redirect_interface(&self, destination_guid.clone(), interface);
            interface.send(packet);
            println!("Packet Redirect: {} => {}, Type: {}", source_guid, destination_guid, message_type)
            
        }


    }

}






// Private Methods
impl ControlHandler {
    fn update_map(&mut self, guid:String, address:(String, String)){
        self.guid_map.insert(guid, address);
    }

    fn get_guids(&self)->Vec<String>{
        let mut guid_list = Vec::new();
        let map = self.guid_map.keys();
        for key in map {
            guid_list.push(key.to_owned());
        }
        return guid_list;
    }

    fn get_address(&self, guid:String)->(String,String){
        let address = self.guid_map.get(&guid)
                                                .expect("GUID to address lookup failed.");
        return address.to_owned();
    }

    fn redirect_interface(&self, guid:String, interface:&mut NetworkInterface){
        let address = Self::get_address(self, guid);
        let _ = interface.set_destination(address);
    }

}
