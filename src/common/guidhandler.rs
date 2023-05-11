use super::netutils::*;
use super::queue::*;
use super::contract::*;
use std::collections::HashMap;
use bincode::deserialize;
use serde::{Serialize,Deserialize};
use bincode::serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RelayType {
    TeamServer { nickname:String },
    Redirector { hostname:String, pid:String },
}

// REMEMBER THIS: If you want to send a guid table to sync it, you should send it as the payload
// of a contract.

// Note that Guids are for interfaces, not for processes.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GuidEntry 
{
    Client{ guid:String, addr:String },
    Endpoint{ guid:String, addr: String, hostname: String, pid:String },
    Relay{ guid:String, addr:String, relaytype:RelayType }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuidTable {
    table: HashMap<String, GuidEntry>,
}

impl GuidTable {
    fn new()->Self{
        let table:HashMap<String, GuidEntry> = HashMap::new();
        GuidTable {
            table
        }
    }
}

/* Different types of handlers */

// All handlers, including the one on a client.
pub trait GuidHandler {
    fn new(interface:NetworkInterface)->Self;
    fn update(&mut self, input_table: GuidTable);
    fn get_table(&self)->Vec<u8>;
}

// Only teamservers/redirectors (Anything that has to perform in-transit analysis on a packet
// destination)
pub trait GuidRelay {
    fn analyze(&mut self, contract:&Contract);
    fn redirect(&self, contract:Contract);

}


/* CLIENT GUID HANDLER */
#[derive(Debug)]
pub struct ClientGuidHandler {
    guid_table: GuidTable,
    pub interface:NetworkInterface,
}

impl GuidHandler for ClientGuidHandler {
    fn new(interface:NetworkInterface)->Self{
        let guid_table = GuidTable::new();
        ClientGuidHandler {
            guid_table,
            interface,
        }
    }

    fn update(&mut self, input_table:GuidTable){
        for key in input_table.table.keys(){
            let entry = input_table.table.get_key_value(key).expect("GUID retrieval failed.");
            let entry_key = *&entry.0;
            let entry_value = entry.1.clone();
            self.guid_table.table.insert(entry_key.to_owned(), entry_value);
        }
    }

    fn get_table(&self)->Vec<u8>{
        bincode::serialize(&self.guid_table.table).expect("GUID Table serialization failed")
    }
}

/* RELAY GUID HANDLER (PS I know I am copy pasting code but I have to access struct fields, so)
 * Should I even be using traits here? Am I stupid? Is this devolving into the same shitcode
 * that I am refactoring in the first place? I don't know and maybe I never will. God help me*/
#[derive(Debug)]
pub struct RelayGuidHandler {
    guid_table: GuidTable,
    pub interface:NetworkInterface,
}

impl GuidHandler for RelayGuidHandler {
    fn new(interface:NetworkInterface)->Self{
        let mut guid_table = GuidTable::new();
        RelayGuidHandler {
            guid_table,
            interface,
        }
    }

    fn update(&mut self, input_table:GuidTable){
        for key in input_table.table.keys(){
            let entry = input_table.table.get_key_value(key).expect("GUID retrieval failed.");
            let entry_key = *&entry.0;
            let entry_value = entry.1.clone();
            self.guid_table.table.insert(entry_key.to_owned(), entry_value);
        }
    }

    fn get_table(&self)->Vec<u8>{
        bincode::serialize(&self.guid_table.table).expect("GUID Table serialization failed")
    }
}

impl GuidRelay for RelayGuidHandler {
    fn analyze(&mut self, contract: &Contract){
        match contract.contract_type {
            ContractType::GuidRequest => {
                let guid_table = self.get_table().to_owned();
                let src_guid = contract.src_guid.clone().expect("GuidRequest lacked source GUID.");
                let mut src_addr = String::new();

                match self.guid_table.table.get(&src_guid).expect("Source GUID not in GUID table."){
                    GuidEntry::Client{addr, ..} => {src_addr = addr.clone()},
                    GuidEntry::Endpoint{addr, ..} => {src_addr = addr.clone()},
                    GuidEntry::Relay{addr, ..}=> {src_addr = addr.clone()},
                }

                let mut guid_sync = Contract::new(
                    src_guid,
                    ContractType::GuidSync,
                    );

                guid_sync.pack_payload(guid_table);

                self.interface.outbound_queue.push((src_addr, guid_sync));
            },
            ContractType::GuidSync => {
                let payload = contract.payload.clone().expect("No payload in GuidSync.");
                let guid_table:GuidTable = deserialize(&payload).expect("Guid table deserialization failed.");
                self.update(guid_table);
            },
            ContractType::GuidPost => {
                println!("GUID POST DETECTED! WOOP WOOP!");
                let payload = contract.payload.clone().expect("No payload in GuidPost");
                let guid_entry:GuidEntry = deserialize(&payload).expect("Guid entry deserialization failed.");
                match guid_entry {
                    GuidEntry::Client{ref guid, ..} => {self.guid_table.table.insert(guid.clone(), guid_entry);},
                    _ => {panic!("NOT IMPLEMENTED")},
                }
            }
            _ => {Self::redirect(&self, contract.clone())}
        }
    }
    fn redirect(&self, contract:Contract){
        let dst_guid = contract.dst_guid.clone();
        if self.guid_table.table.contains_key(&dst_guid){
            let guid_entry = self.guid_table.table.get(&dst_guid).expect("dst_guid lookup failed.");
            match guid_entry {
                GuidEntry::Client{addr, ..} => {self.interface.outbound_queue.push((addr.clone(), contract))},
                GuidEntry::Endpoint{addr, ..} => {self.interface.outbound_queue.push((addr.clone(), contract))},
                GuidEntry::Relay {addr, ..} => {self.interface.outbound_queue.push((addr.clone(), contract))},
            }
        }
    }
}

/* Right now these literally just fill the dst_guid with nothing, but this whole thing
 * honestly needs to be refactored pretty severely.*/
pub fn client_guid_post(addr:String, guid:String)->Contract{
    let mut post_contract = Contract::new(
        "".to_string(),
        ContractType::GuidPost,
        );
    let guid_entry = GuidEntry::Client {addr, guid};
    let payload = bincode::serialize(&guid_entry).expect("Failed to serialize guidentry");
    post_contract.pack_payload(payload);
    post_contract
}
