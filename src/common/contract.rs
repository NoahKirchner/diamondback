use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug, Clone)]
pub struct Contract {
    pub src_guid: Option<String>,
    pub dst_guid: String,
    // BLA BLA BLA make invalid states unrepresentable
    pub contract_type: ContractType,

    pub command: Option<String>,
    // Serialized data
    pub payload:  Option<Vec<u8>>,
    // Any additional data
    pub data: Option<Vec<String>>,

}

// Eventually make sure that you add stuff for GUID updating
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ContractType {
    Command,
    Response,
    GuidRequest,
    GuidSync,
    GuidPost,
}

impl Contract {
    pub fn new(dst_guid: String, contract_type: ContractType)->Self{
        // src_guid filled in by 
        let mut src_guid = None;
        let mut command = None;
        let mut payload = None;
        let mut data =  None;
        Contract {
            src_guid,
            dst_guid,
            contract_type,
            command,
            payload,
            data,
        }
    }

    pub fn pack_command(&mut self, command: String){
        self.command = Option::from(command);
    }

    pub fn pack_payload(&mut self, payload: Vec<u8>){
        self.payload = Option::from(payload);
    }

    pub fn pack_data(&mut self, data:Vec<String>){
        self.data = Option::from(data);

    }
    pub fn pack_src_guid(&mut self, src_guid:String){
        self.src_guid = Option::from(src_guid);
    }


}
