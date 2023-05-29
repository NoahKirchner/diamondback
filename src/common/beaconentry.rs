
use super::{contract::*, queue::{Queue, QueueMethods}};
use crate::common::beaconentry::BeaconData::*;

pub struct BeaconEntry {
    nickname: String,
    source_ip: String,
    port: u32,
    host_data: HostData,
    process_data: ProcessData,
    configs: BeaconConfigs,
    outbound_contracts: Queue<Contract>,
    // Add task database here. (Or maybe don't? This might be better suited for somewhere else)
    // @TODO
}

impl<'a> BeaconEntry {
    pub fn new(nickname:String, source_ip:String, port:u32)->Self{
        BeaconEntry{
            nickname,
            source_ip,
            port,
            host_data: HostData::new(),
            process_data: ProcessData::new(),
            configs: BeaconConfigs::new(),
            outbound_contracts: Queue::<Contract>::new(),

        }
    }

    // Sets or gets values (as opposed to structs) in the entry.

    pub fn set_nickname(mut self, nickname:String){
        self.nickname = nickname
    }

    pub fn get_nickname(self)->String{
        self.nickname
    }

    pub fn set_source_ip(mut self, source_ip:String){
        self.source_ip = source_ip
    }

    pub fn get_source_ip(self)->String{
        self.source_ip
    }

    pub fn set_port(mut self, port:u32){
        self.port = port
    }

    pub fn get_port(self)->u32{
        self.port
    }

    // Gets references to structs contained in the entry or replaces the struct. The idea here is
    // that the a contract might contain a serialized version of one of these data structures 
    // instead of just sending a bunch of random data to parse through.
    pub fn get_host_data(&'a self)->&'a HostData{
        &self.host_data
    }

    pub fn set_host_data(mut self, host_data:HostData){
        self.host_data = host_data;
    }

    pub fn get_process_data(&'a self)->&'a ProcessData{
        &self.process_data
    }

    pub fn set_process_data(mut self, process_data:ProcessData){
        self.process_data = process_data;
    }

    pub fn get_configs(&'a self)->&'a BeaconConfigs{
        &self.configs
    }

    pub fn set_configs(mut self, configs:BeaconConfigs){
        self.configs = configs;
    }


    // No setter for this because it should never be replaced.
    pub fn get_contract_queue(&'a self)->&'a Queue<Contract>{
        &self.outbound_contracts
    }

    // Abstraction functions to make accessing frequent internal data easier (specifically contract
    // queue). These are basically wrappers.
    
    pub fn pop_contract(mut self)->Contract{
        self.outbound_contracts.pop()
    }

    pub fn push_contract(mut self, contract:Contract){
        self.outbound_contracts.push(contract);
    }

    // Functions to allow data structures to be updated from beacon.


}



pub mod BeaconData {
use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct HostData {
        hostname: String,
        operating_system: String,
    }

    impl HostData {
        pub fn new()->Self{
            HostData {
                hostname: String::new(),
                operating_system: String::new(),
            }
        }

        pub fn set_hostname(mut self, hostname:String){
            self.hostname = hostname;
        }

        pub fn get_hostname(self)->String{
            self.hostname
        }

        pub fn set_operating_system(mut self, operating_system:String){
            self.operating_system = operating_system;
        }

        pub fn get_operating_system(self)->String{
            self.operating_system
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct ProcessData {
        pid: u32,
        process_name: String,
        user_data: UserData,
    }

    impl ProcessData {
        pub fn new()->Self{
            ProcessData {
                pid: 0,
                process_name: String::new(),
                user_data: UserData::new(),
            }
        }

        pub fn set_pid(mut self, pid:u32){
            self.pid = pid
        }

        pub fn get_pid(self)->u32{
            self.pid
        }

        pub fn set_process_name(mut self, process_name: String){
            self.process_name = process_name;
        }

        pub fn get_process_name(self)->String{
            self.process_name
        }

    }

    #[derive(Serialize, Deserialize)]
    pub struct UserData {
        username: String,
        group: String,
        permissions: String,
    }

    impl UserData {
        pub fn new()->Self{
            UserData {
                username: String::new(),
                group: String::new(),
                permissions: String::new(),
            }
        }

        pub fn set_username(mut self, username:String){
            self.username = username;
        }

        pub fn get_username(self)->String{
            self.username
        }

        pub fn set_group(mut self, group: String){
            self.group = group;
        }

        pub fn get_group(self)->String{
            self.group
        }

        pub fn set_permissions(mut self, permissions: String){
            self.permissions = permissions;
        }

        pub fn get_permissions(self)->String{
            self.permissions
        }


    }

    // @TODO Turn these into time values. This also oughta have a default state set somewhere.
    #[derive(Serialize, Deserialize)]
    pub struct BeaconConfigs {
        delay: u32,
        jitter: u32,
    }

    impl BeaconConfigs {
        pub fn new()->Self{
            BeaconConfigs {
                delay: 0,
                jitter: 0,
            }
        }

        pub fn set_delay(mut self, delay:u32){
            self.delay = delay;
        }

        pub fn get_delay(self)->u32{
            self.delay
        }

        pub fn set_jitter(mut self, jitter:u32){
            self.jitter = jitter;
        }

        pub fn get_jitter(self)->u32{
            self.jitter
        }
    }

}

