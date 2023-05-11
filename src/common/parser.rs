use super::contract::*;
use super::commandutils::*;

pub struct Parser {
    manager: CommandManager,
}

#[derive(Debug)]
pub enum ParsedValue{
    ParsedCommand{response:Contract},
    ParsedResponse{result:Vec<String>, source:Option<String>},
    Other{contract:Contract},
}

impl Parser {
    pub fn new(guid: String)->Self{
        let manager = CommandManager::new();
        Parser {
            manager,
        }
    }

    // Handling incoming contracts.
    
    pub fn parse(&self, contract: Contract)->ParsedValue{
        match contract.contract_type {
            ContractType::Command => {Self::handle_command(&self, contract)},
            ContractType::Response => {Self::handle_response(contract)},
            ContractType::GuidSync => {ParsedValue::Other{contract}},
            ContractType::GuidRequest => {ParsedValue::Other{contract}},
            ContractType::GuidPost => {ParsedValue::Other{contract}},
        }
    }

    fn handle_command(&self, contract: Contract)->ParsedValue{
        let command = contract.command.expect("Parser found no command string");
        // Performs a lookup and returns a reference to the correct function.
        let function = self.manager.command_list.get(&command).expect("failed command table unwrap");
        // Calls the function from above and saves the output.
        let result = (*function)();

        let response = Self::build_contract(
            &self,
            "FIXME".to_string(),
            ContractType::Response,
            None,
            None,
            Option::from(result),
            );

        ParsedValue::ParsedCommand{response}

        // functionality to come. oh yeah. swag.
    }

    fn handle_response(contract: Contract)->ParsedValue{
        let result = contract.data.to_owned().expect("Response data failed to unwrap");
        let source = contract.src_guid.to_owned();
        ParsedValue::ParsedResponse{result, source}
    }

    // Handling outgoing contracts
    
    pub fn build_contract(&self,
        dst_guid: String,
        contract_type: ContractType,
        command: Option<String>,
        payload: Option<Vec<u8>>,
        data: Option<Vec<String>>)->Contract
    {

        let mut contract = Contract::new(dst_guid, contract_type);
        if command.is_some() {
            contract.pack_command(command.expect("Parser command pack failed"));
        }
        if payload.is_some(){
            contract.pack_payload(payload.expect("Parser payload pack failed"));
        }
        if data.is_some(){
            contract.pack_data(data.expect("Parser data pack failed"));
        }
        contract
     
    }

}
