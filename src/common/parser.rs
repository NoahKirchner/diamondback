
use std::{collections::{hash_map::DefaultHasher}, time::SystemTime, hash::{Hasher, Hash}, rc::Rc};

use super::protocol::*;

use bincode::{*, config::Infinite};

use serde::Deserialize;

use super::command_types::*;

const GUID_POST:&str = "guid_post";
const GUID_REQUEST:&str = "guid_request";
const RESPONSE:&str = "response";
const COMMAND:&str = "command";

pub trait ParserMethods {
    // Constructor. Generates the source GUID for the binary.
    fn new()->Self;

    // Ingests input and packs a packet.
    fn new_payload(&self,
                    dst_guid:Option<String>,
                    payload:Vec<u8>,
                    )->Packet;

    fn new_response(&self,
                    dst_guid:Option<String>,
                    response:Vec<String>,
                    )->Packet;

    fn new_guid_post(&self, address:String)->Packet;

    fn new_guid_request(&self)->Packet;
    
    // Ingests a packet and returns a tuple of the source GUID, type & payload.
    fn ingest_payload(&self, packet:Packet)->(Option<String>, String, Vec<u8>);

    // Ingests a packet and returns a tuple of the source GUID, type and response strings.
    fn ingest_response(&self, packet:Packet)->(Option<String>, String, Vec<String>);

    // Returns the GUID for the binary.
    fn get_guid(&self)->String;

}

pub struct Parser{
    src_guid:String
}

impl ParserMethods for Parser {

    fn new()->Self{
        let src_guid = Self::generate_guid();
        Self {
            src_guid,
        }
    }

    fn new_payload(&self,dst_guid:Option<String>, payload:Vec<u8>)->Packet {

        let mut outbound_packet = Packet::new(COMMAND.to_string());

        outbound_packet.pack_source(self.src_guid.clone());

        if dst_guid.is_some(){
            // This is not unsafe because of above ^
            outbound_packet.pack_destination(dst_guid.expect("Payload GUID unwrap failed."));
        }

        outbound_packet.pack_payload(payload);
        
        return outbound_packet;
    }

    fn new_response(&self,dst_guid:Option<String>, response:Vec<String>)->Packet {
            let mut response_packet = Packet::new(RESPONSE.to_string());
            response_packet.pack_response(response);
            response_packet.pack_source(self.src_guid.clone());
            if dst_guid.is_some(){
                response_packet.pack_destination(dst_guid.expect("Response GUID unwrap failed."));
            }
            return response_packet;
        }

    // This needs badly unfucked, holy shit.
    fn new_guid_post(&self, address:String)->Packet{
        let mut guid_packet = Packet::new(GUID_POST.to_string());
        guid_packet.pack_source(self.src_guid.clone());
        guid_packet.pack_response(vec![address]);
        return guid_packet;
    }

    fn new_guid_request(&self)->Packet{
        let mut guid_request = Packet::new(GUID_REQUEST.to_string());
        guid_request.pack_source(self.src_guid.clone());
        return guid_request;
    }

    fn ingest_payload(&self, packet:Packet)->(Option<String>, String, Vec<u8>) {
        let src_guid = packet.get_source_guid();

        let message_type = packet.get_message_type();

        let payload = packet.get_payload();

        return (src_guid, message_type, payload);
    }

    fn ingest_response(&self, packet:Packet)->(Option<String>,String,Vec<String>) {
        let src_guid = packet.get_source_guid();

        let message_type = packet.get_message_type();

        let response = packet.get_response();

        return (src_guid, message_type, response);
    }

    fn get_guid(&self)->String{
        self.src_guid.clone()
    }

    


}

impl Parser {
    fn generate_guid()->String{
        let source = SystemTime::now();
        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        let guid = hasher.finish().to_string();
        guid
    }

    // Verifies that the input implements the Armory trait.
    pub fn serialize_payload<T: CommandType>(payload_object:T)->Vec<u8>{
        let command = payload_object;
        let encoded = serialize(&command)
                                .expect("Command failed to serialize.");
        return encoded;
 
    }

    pub fn deserialize_payload(serialized_payload:Vec<u8>)-> CommandEnum{
        let decoded = deserialize(&serialized_payload[..]);
        let commandenum = decoded.expect("Command deserialization failed.");
        return commandenum;
    }
}