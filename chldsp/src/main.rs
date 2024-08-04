use std::env::Args;
use std::process::{exit, Command};
use std::os::unix::process::CommandExt;

fn run(mut args: Args) -> i32 {
    match args.nth(1) {
        None => {
            eprintln!("No command specified");
            return 1;
        },
        Some(ref command) => {
            let args: Vec<String> = args.collect();
            // Print full command line
            // eprintln!("Command line: {} {:?}", command, args);
            let err = Command::new(command)
                .args(&args)
                .exec();
            eprintln!("Failed to execute command: {}", err);
            return 1;
        },
    }
}

fn main() {
    exit(run(std::env::args()));
}
