extern crate libc;

use std::ffi::CString;
use libc::{c_int, chroot};

use anyhow::Result;
use std::io::Write;

use std::string::String;

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...


pub fn chroot_directory(path: &str) -> Result<(), String> {
    let path_c = CString::new(path).map_err(|_| "CString conversion failed")?;

    let result = unsafe {
        chroot(path_c.as_ptr())
    };

    if result == 0 {
        Ok(())
    } else {
        Err("chroot failed".to_string())
    }
}

fn main() -> Result<()> {

    let args: Vec<_> = std::env::args().collect();
    let path = "./chroot";
    
    chroot_directory(path).ok();

    // path of the docker executable
    let command = &args[3];

    // arguments to the docker executable
    let command_args = &args[4..];

    // run the docker command
    let output = std::process::Command::new(command)
        .args(command_args)
        .output()?;

    // print the output of the docker command
    std::io::stdout().write_all(&output.stdout)?;
    std::io::stderr().write_all(&output.stderr)?;

    // exit with the same exit code as the docker command
    std::process::exit(output.status.code().unwrap_or(1));
}