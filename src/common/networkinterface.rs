/*
Network Interface Object

This object is intended to handle all networking operations
to ensure that every part of the platform has the same capabilities
and low level configurations.

Effectively this is a handler for a TcpListener and TcpStream object.

*/

// TCP functionality.
use std::net::{TcpListener, TcpStream};

// Writing & Reading from stream and WouldBlock error catching.
use std::io::{self, Write, Read};
use std::sync::Arc;
use std::thread::{sleep, self};
use std::time::Duration;

// Serialization
use bincode::*;

// Packet object
use super::protocol::*;
// Queue object
use super::queue::*;

// @TODO STOP USING TUPLES FOR IP ADDRESSES IT'S DUMB AND NOW A PAIN IN MY ASS


// Universal chunk size.
const CHUNKSIZE: usize = 512;

pub trait NetworkingMethods {

    // Constructor
    fn new(source:(String,String), destination:(String,String))->Self;
    // A loop for listening for packets that will push into a Queue object (queue.rs)
    fn listen(&self, queue:Arc<Queue<Packet>>);
    // Will send a constructed packet.
    fn send(&self,packet:Packet);
    
    /*
    source/destination IP handlers:

    I know it does not make much sense to add a mutate method just for 
    changing information when you could just feed it to the method every
    time, but the manager and agent will always be sending information
    to the same IP addresses, the only one that needs to change is the
    server. The server's source IP address will never need to change, 
    but the agent's might for dynamic resolution or what have you, so
    I decided to do this as a make-code-look-slightly-nicer solution.

    I will probably refactor it later if necessary.
    */
    fn set_source(&mut self, address:(String,String));

    fn set_destination(&mut self, address:(String,String));

    // Later probably add a way to check the current destinations.

}

#[derive(Clone)]
pub struct NetworkInterface{
    // The source IP address.
    src:String,
    // The destination IP address. (Should be the C2 server and not the agent).
    dst:String,
}

impl NetworkingMethods for NetworkInterface{
    fn new(source:(String,String), destination:(String,String))->Self{
        let src: String = Self::create_address(source);
        let dst: String = Self::create_address(destination);
        Self {
            src,
            dst,
        }
    }

    fn listen(&self, queue:Arc<Queue<Packet>>){
        let mut connection: TcpListener = TcpListener::bind(&self.src)
                            .expect("Failed to bind to source port.");
        
        connection.set_nonblocking(true)
                .expect("Failed to set non-blocking on listener.");

        for stream in connection.incoming(){
            match stream {

                // Passes successful stream to handle_stream and pushes packet to queue.
                Ok(success) => {
                    let inbound = Self::handle_stream(success)
                            .expect("handle_stream failed in listen loop.");
                    
                    queue.push(inbound);
                }
                
                // Do not panic if no connection is received.
                Err(ref error) if error.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }

                Err(error) => panic!("Listener stream IO error: {}", error),
            }


        }
        
    }

    fn send(&self, outbound:Packet){
        loop {
            match TcpStream::connect(&self.dst) {
                Ok(success) => {
                    Self::handle_send(success, outbound);
                    break;
                }
                // Do not panic if no connection can be made, keep trying.
                Err(ref error) if error.kind() == io::ErrorKind::ConnectionRefused => {
                    continue;
                }
                Err(error) => panic!("Send stream IO error: {}", error)
            }
        }
    }

    fn set_source(&mut self, address:(String,String)) {
        self.src = Self::create_address(address);
    }

    fn set_destination(&mut self, address:(String,String)) {
        self.dst = Self::create_address(address);

    }


}

// Private helper functions that don't need to be exposed.
impl NetworkInterface{
    // Combines an IP and Port into the format that the TCP listener
    // and stream want.
    // @TODO Why even have these in a tuple ??
    fn create_address(input:(String,String))->String{
        let address = input.0 + ":" + input.1.as_str();
        return address;
    }

    // Converts from a packet object to a byte vector.
    fn serialize_packet(packet:Packet)->Vec<u8>{
        let encoded: Vec<u8> = bincode::serialize(&packet)
                                .expect("Serialize packet failed.");
        return encoded;
    }

    // Converts a byte vector to a packet object.
    fn deserialize_packet(packet:&Vec<u8>)->Packet{
        let decoded: Packet = deserialize(&packet[..])
                            .expect("Deserialize packet failed.");
        return decoded;
    }

    // Takes in a byte vector and returns an iterator of that 
    // byte vector, allowing it to be sent in sequence. Also ensures
    // that each item in the iterator does not outlive the duration of the
    // function.
    fn chunk_packet<'a>(encoded_packet:&'a [u8])-> impl Iterator<Item = &'a [u8]>{
        encoded_packet.chunks(CHUNKSIZE)
    }

    // Handles packet serialization, chunking and writing to stream.
    fn handle_send(mut stream:TcpStream, outbound:Packet){
        let mut serialized_packet: Vec<u8> = Self::serialize_packet(outbound);
        let mut packet_chunks = Self::chunk_packet(&serialized_packet);

        while let Some(chunk) = packet_chunks.next(){
            // Writes each chunk in the iterator to the stream.
            let _ = stream.write_all(chunk);
        }

    }

    // Handles stream reading, dechunking and serialization.
    fn handle_stream(mut stream:TcpStream)->Option<Packet>{
        // u8 (byte) vector at a fixed size.
        let mut buffer = [0; CHUNKSIZE];
        let mut received_chunks:Vec<u8> = Vec::new();

        loop {
            sleep(Duration::new(0,1));
            // Tries to read any inbound bytes to buffer.
            match stream.read(&mut buffer){

                Ok(read) => {
                    // Break at end of packet.
                    if read == 0 {
                        break None;
                    }

                    // Create a byte vector with each received chunk.
                    received_chunks.extend_from_slice(&buffer[..read]);

                    let deserialized_packet: Packet = Self::deserialize_packet(&received_chunks);

                    stream.flush()
                        .expect("Stream flush failed.");
                    
                    return Some(deserialized_packet);

                }

                // Stops panic when there are no bytes received so the connection
                // can stay open indefinitely.
                Err(ref error) if error.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }

                Err(error) => panic!("Stream buffer read error: {}", error)
            }

        }

    }
}

/*
Networking thread helpers.

These are two public helper functions that just make it easier to spawn threads.

*/

// Clones an interface and sets up a listener in another thread.
pub fn listen_thread(interface: &NetworkInterface, inbound_queue:&Arc<Queue<Packet>>) {
    let interface_clone = interface.clone();
    let queue_clone = inbound_queue.clone();
    let mut listen_thread = thread::spawn(move || {
        interface_clone.listen(queue_clone);
    });
}

// Clones an interface, pulls packets from a queue and sends them in a thread.
pub fn send_thread(interface:&NetworkInterface, outbound_queue:&Arc<Queue<Packet>>) {
    let interface_clone = interface.clone();
    let queue_clone = outbound_queue.clone();
    let mut input_thread = thread::spawn(move || {
        loop {
            if queue_clone.is_empty(){
                sleep(Duration::new(0,50000));
                continue;
            }
            else {
                interface_clone.send(queue_clone.pop());
            }
        }
    });
}



