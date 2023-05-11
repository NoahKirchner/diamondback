// Eventually turn this into a directory and split it. 
use std::collections::HashMap; 
use serde::{Serialize, Deserialize};

use telemetry_commands::*;
// A struct that implements a map to a box
// of all desired beacon commands.

pub struct CommandManager {
    pub command_list: HashMap<String, Box<dyn Fn()->Vec<String>>>

}

impl CommandManager {
    pub fn new()->CommandManager{
        // \/ For handling optional additions
        //#[cfg(feature="feature")]

        // If at any point you need to determine whether or not to include multiple commands
        // with one feature, just have it be done through a function instead.
        let mut command_list:HashMap<String, Box<dyn Fn()->Vec<String>>> = HashMap::new();
        command_list.insert("get_pid".to_string(), Box::new(get_pid));    
        command_list.insert("ping".to_string(), Box::new(ping));

        CommandManager {
            command_list
        }

    }

}

pub mod telemetry_commands {
    use std::process;


pub fn get_pid()->Vec<String>{
    let pid = process::id();
    return vec!["PID".to_string(), pid.to_string()]
}

pub fn ping()->Vec<String>{
    return vec!["pong!".to_string()]
}

}

pub mod shell_commands {

/*===This needs to be re-written===

    // eventually, turn the 'sh' mention into an argument
    fn execute_sh_script()->Vec<String> {
        let mut shell = Command::new("sh");

        shell.stdout(Stdio::piped());
        shell.stdin(Stdio::piped());

        let mut subprocess = shell.spawn().expect("Subprocess failed to spawn.");

        let mut subprocess_stdin = subprocess.stdin.take()
                                                .expect("STDIN take failed.");

        subprocess_stdin.write_all(&self.scrip)


    }
*/

}
