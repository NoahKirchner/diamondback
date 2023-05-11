/*
Network Utility Module 
 */
use std::net::{TcpListener, TcpStream};
use std::io::{self, Write, Read};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::process::id;
use std::mem::transmute;
use std::thread::{sleep, self};
use std::sync::Arc;
use super::queue::*;
use super::contract::*;


use bincode::*;

const CHUNKSIZE: usize = 512;


// Note: The reason that IP addresses here are explicitly stated is to allow
// for TCP pivoting to be configured.
#[derive(Debug)]
pub struct NetworkInterface {
    // We want these to be public to allow other functions to add contracts to be sent and extract 
    // contracts that have been received for parsing.
    pub inbound_queue:Arc<Queue<Contract>>,
    pub outbound_queue:Arc<Queue<(String,Contract)>>,
    pub listeners:Vec<(String, String, thread::JoinHandle<()>)>,
    pub streams:Vec<(String, thread::JoinHandle<()>)>,
}

impl NetworkInterface {
    pub fn new()->Self{
        let inbound_queue = Arc::new(Queue::new());
        let outbound_queue = Arc::new(Queue::new());
        let mut listeners = Vec::new();
        let mut streams = Vec::new();
        NetworkInterface{
            inbound_queue,
            outbound_queue,
            listeners,
            streams,
        }
    }

    pub fn create_listener(&mut self, lhost:String)->(String, String){
        let mut listener = TcpChannelListener::new(lhost.clone(), &self.inbound_queue);
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let nanoseed = timestamp.as_nanos() as u32;
        let microseed = timestamp.as_micros() as u32;
        let guid = format!("{:08x}-{:08x}", nanoseed, microseed );
        let handle = listener.spawn();
        self.listeners.push((handle.0, guid.clone(), handle.1));
        (lhost, guid)
        // At some point, probably do not have the listening/streaming start at creation incase
        // you want dynamic resolution or TCP pivoting or something.
    }

    pub fn create_stream(&mut self, rhost:String){
        let mut stream = TcpChannelStream::new(rhost, &self.outbound_queue);
        let handle = stream.spawn();
        self.streams.push(handle);
    }

    // This is potentially the fucking dumbest thing I have ever written but it 
    // somehow works. Basically, it drains the streams vector into an interator and 
    // iterates over it, joining anything that matches rhost. It then dumps anything that 
    // does not match that into a buffer and then re-fills (by assignment) the streams 
    // vector.
    pub fn join_stream(&mut self, rhost:String){
        let mut return_streams = Vec::new();
        for stream in self.streams.drain(..){
            if stream.0 == rhost {
                stream.1.join();
            }
            else {
                return_streams.push(stream);
            }
        }
        self.streams = return_streams;
    }

    pub fn join_listener(&mut self, lhost:String){
        let mut return_listeners = Vec::new();
        for listener in self.listeners.drain(..){
            if listener.0 == lhost {
                listener.2.join();
            }
            else {
                return_listeners.push(listener);
            }
        }
        self.listeners = return_listeners;
    }
}

// Whenever you implement new protocols, you need to give these wrappers
// a trait so that they can be used interchangably in the above vectors.
#[derive(Debug)]
struct TcpChannelListener {
    lhost:String,
    queue:Arc<Queue<Contract>>
}

// Keep in mind that if 0 is supplied to tcplistener bind it assigns a random high
impl TcpChannelListener {
    pub fn new(lhost:String,queue:&Arc<Queue<Contract>>)->Self{
        let queue_clone = queue.clone();
        TcpChannelListener{
            lhost,
            queue: queue_clone,
        }
    }

    pub fn spawn(self)->(String, thread::JoinHandle<()>){
        let lhost = self.lhost.clone();
        let handle = thread::spawn(move || {self.listen()});
        (lhost, handle)
    }

    fn listen(self){
        let mut connection: TcpListener = TcpListener::bind(self.lhost).expect("Failed TCP Bind");
        connection.set_nonblocking(true).expect("Failed to set nonblocking");

        for stream in connection.incoming() {
            match stream {
                Ok(success) => {
                    let inbound = Self::handle(success).expect("Failed to handle success");
                    self.queue.push(inbound);
                }
                // Do not panic if no connection.
                Err(ref error) if error.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(error) => panic!("Listener stream IO error: {}", error),
            }
        }

    }

    // Takes the stream of bytes and regularizes them back into a contract.
    fn handle(mut stream:TcpStream)->Option<Contract>{
        // u8 vector at a fixed size? preposterous. also assign this to a constant later
        let mut buffer = [0; CHUNKSIZE];
        let mut received_chunks:Vec<u8> = Vec::new();

        loop {
            sleep(Duration::new(0,1));
            match stream.read(&mut buffer){
                Ok(read) => {
                    // Break at end of packet
                    if read == 0 {
                        break None;
                    }

                    received_chunks.extend_from_slice(&buffer[..read]);
                    let decoded = deserialize(&received_chunks[..]).unwrap();
                    stream.flush().expect("Stream flush failed.");
                    return Some(Contract::from(decoded));
                }
                // Doesn't panic if the stream doesn't receive a message.
                Err(ref error) if error.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(error) => panic!("Stream buffer read error: {}", error)
            }
        }
    }
    
}

// TCP Stream/Send object. Takes in items from a queue and sends them to a destination.
#[derive(Debug)]
struct TcpChannelStream{
    rhost:String,
    queue:Arc<Queue<(String, Contract)>>,
}

impl TcpChannelStream{
    // Creates a new stream and prepares it to be threaded.
    pub fn new(rhost:String, queue:&Arc<Queue<(String, Contract)>>)->Self{
        let queue_clone = queue.clone();
        TcpChannelStream {
            rhost,
            queue: queue_clone,
        }
    }
    // Spawns a new thread and moves itself into it, returning a handle.
    pub fn spawn(self)->(String, thread::JoinHandle<()>){
        let rhost = self.rhost.clone();
        let handle = thread::spawn(move || {self.send()});
        (rhost, handle)
    }

    pub fn send(&self){
        loop {
            if !self.queue.is_empty(){
                // Check to see if this is the correct destination.
                if self.queue.peek().0 != self.rhost{
                    continue;
                }
                match TcpStream::connect(&self.rhost) {
                    Ok(success) => {
                        Self::handle(success, self.queue.pop().1);
                    }
                    // Do not panic on connection refused.
                    Err(ref error) if error.kind() == io::ErrorKind::ConnectionRefused => {
                        continue;
                    }
                    Err(error) => panic!("Send stream IO error: {}", error)
                }
            }
        }

    }
    
    // Mostly serializes and chunks the contract for easy sending.
    pub fn handle(mut stream:TcpStream, contract:Contract){
        let mut serialized_contract: Vec<u8> = bincode::serialize(&contract).expect("Contract serialization failed");
        let mut contract_chunks = serialized_contract.chunks(CHUNKSIZE);

        while let Some(chunk) = contract_chunks.next(){
            let _ = stream.write_all(chunk);
        }
    }
}
