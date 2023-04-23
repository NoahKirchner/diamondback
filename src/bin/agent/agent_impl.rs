use std::{collections::HashMap, io}; // Remember to remove io


use crate::{Agent, armory::*, common::NetworkInterface, queue};

use queue::*;


impl Agent<'_> {

    pub fn new(source:(String,String),destination:(String,String))-> Self{


        // This took an inordinately long amount of time until
        // I remembered that * existed. This is a hashmap
        // with a String as the key and the value is a pointer
        // to a method reference. I don't know if this is going
        // to work but it's worth a shot.
        let mut command_list:HashMap<String, Box<fn(&Self)->()>> = HashMap::new();

        // Does the insertion with a String for the key and a box with a method reference for the value.
        command_list.insert("debug".to_string().to_owned(), Box::new(*&Self::debug_command));
        // ^^^ eventually this will be deserialized from somewhere, god willing.

        

        Self {
            command_list,
            iopipe:Queue::new(),
            argument:None,
            // Clones them so the arguments can be passed to the network interface.
            source: source.clone(),
            destination: destination.clone(),
            interface:NetworkInterface::new(source,destination),


        }
    }


    // Main execution loop. Very basic for right now.
    
    pub fn execute(&mut self){

        let mut line = String::new();


        loop {
            
            // Input testing for method execution.
            println!("Input (Debug):");
            io::stdin()
                .read_line(&mut line)
                .expect("Invalid input");
            line.strip_suffix("\r\n")
                .expect("Invalid transformation")
                .to_string();

            println!("Input was {}", line);

            // The strip suffix is to remove the carriage return and newline,
            // since this won't be reading user input when it actually works
            // it is a hackjob fix.
            self.iopipe.push(line.strip_suffix("\r\n").expect("Oh shit").to_string());

            line.clear();

            // Test for popping items in and out of the queue.

            let pipeout = self.iopipe.pop();

            println!("{}", pipeout);
            // Searches hashmap for the key input to find the related method.
            let search = self.command_list.get(&pipeout)
                .expect("No command found");
            
            // Executes the method as a reference swag mode.
            let command = search.as_ref();
            command(&self);



        }


        }
        


    }



