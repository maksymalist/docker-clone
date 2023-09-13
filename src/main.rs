use anyhow::{Context, Result};
use std::io::Write;
// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> Result<()> {

    let args: Vec<_> = std::env::args().collect();

    // path of the docker executable
    let command = &args[3];

    // arguments to the docker executable
    let command_args = &args[4..];

    // run the docker command
    let output = std::process::Command::new(command)
        .args(command_args)
        .output()
        .with_context(|| {
            format!(
                "Tried to run '{}' with arguments {:?}",
                command, command_args
            )
        })?;

    // print the output of the docker command
    if output.status.success() {
        std::io::stdout().write_all(&output.stdout)?;
        std::io::stderr().write_all(&output.stderr)?;
    } else {
        std::process::exit(1);
    }
    Ok(())
}