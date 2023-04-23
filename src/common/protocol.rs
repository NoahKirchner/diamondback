/*
Packet object is defined here. Packet construction is handled by
the parser, not here, this just handles the packet struct and 
basic manipulation capabilities.
*/

// Required for the derives to serialize the packet in networkinterface.
use serde::{Serialize,Deserialize};


pub trait PacketMethods{

    // Constructor, packs the packet.
    fn new(message_type:String)->Self;

    // Used to unpack the packet.
    fn get_source_guid(&self)->Option<String>;

    fn get_destination_guid(&self)->Option<String>;

    fn get_message_type(&self)->String;

    fn get_payload(&self)->Vec<u8>;

    fn get_response(&self)->Vec<String>;


    // Used to pack data into the packet.
    fn pack_source(&mut self,guid:String);

    fn pack_destination(&mut self,guid:String);

    fn pack_payload(&mut self, payload:Vec<u8>);

    fn pack_response(&mut self, response:Vec<String>);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    // Source identifier (to keep IP addresses anonymous)
    src_guid:       Option<String>,
    // Destination identifier (to keep IP addresses anonymous)
    dst_guid:       Option<String>,
    // Message type (Defined in parser)
    message_type:   String,
    // Serialized command.
    payload: Vec<u8>,
    // Arbitrary String values for packet (Handled in parser).
    response:        Vec<String>,
}

impl PacketMethods for Packet {

    fn new(
        message_type:String,
    )->Self{

        let mut payload:Vec<u8> = Vec::<u8>::new();
        let mut response:Vec<String> = Vec::<String>::new();
        Self {
            src_guid:None,
            dst_guid:None,
            message_type,
            payload,
            response,
        }
    }

    fn get_message_type(&self)->String {
        return self.message_type.clone();
    }

    fn get_source_guid(&self)->Option<String> {
        return self.src_guid.clone();
    }

    fn get_destination_guid(&self)->Option<String> {
        return self.dst_guid.clone();
    }

    fn get_payload(&self)->Vec<u8>{
        return self.payload.clone();
    }

    fn get_response(&self)->Vec<String>{
        return self.response.clone()
    }

    fn pack_source(&mut self,guid:String) {
        self.src_guid.insert(guid);
    }

    fn pack_destination(&mut self,guid:String) {
        self.dst_guid.insert(guid);
    }

    fn pack_payload(&mut self, into_payload:Vec<u8>){
        for byte in into_payload {
            self.payload.push(byte);
        }

    }

    fn pack_response(&mut self, into_response:Vec<String>) {
        for item in into_response {
            self.response.push(item);
        }
    }


}