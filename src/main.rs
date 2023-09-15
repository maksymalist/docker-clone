extern crate libc;
use anyhow::{Context, Result};
// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let command = &args[3];
    let command_args = &args[4..];
    // Create a temproty directory
    let temp_dir = tempfile::tempdir()?;
    let dir_path = temp_dir.path().to_path_buf();
    // Copy command to the temporary directory
    let dest_path = temp_dir.path().join(command.strip_prefix("/").unwrap());
    if let Some(parent) = dest_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::copy(command, dest_path)?;
    let c_dir_path = std::ffi::CString::new(dir_path.to_str().unwrap()).unwrap();
    // chroot into the temporary directory
    unsafe {
        libc::chroot(c_dir_path.as_ptr());
    }
    // Create empty /dev/null
    std::fs::create_dir_all("/dev")?;
    std::fs::File::create("/dev/null")?;
    let output = std::process::Command::new(command)
        .args(command_args)
        .output()
        .with_context(|| {
            format!(
                "Tried to run '{}' with arguments {:?}",
                command, command_args
            )
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    print!("{}", stdout);
    eprint!("{}", stderr);

    std::process::exit(output.status.code().unwrap_or(1));
}