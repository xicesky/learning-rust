use pwhash::bcrypt;
use std::env::Args;
use std::process::{exit, Command};
use std::os::unix::process::CommandExt;

static HASH: &str = "";

fn check_and_exec(command: &String, args: Vec<String>) -> i32 {
    match rpassword::prompt_password("Password: ") {
        Err(_) => {
            eprintln!("Failed to read password");
            return 1;
        },
        Ok(password) => {
            if !bcrypt::verify(password, HASH) {
                eprintln!("Invalid password");
                return 1;
            }

            let err = Command::new(command)
                .args(&args)
                .exec();
            eprintln!("Failed to execute command: {}", err);
            return 1;
        },
    }
}

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
            return check_and_exec(command, args);
        },
    }
}

fn main() {
    exit(run(std::env::args()));
}
