use rand::Rng;
use std::{
    io::{stdin, stdout, Write},
    process::Command,
};

fn rm(is_sudo: bool, args: &[&str]) -> Result<(), String> {
    let mut cmd = Command::new("rm");
    if is_sudo {
        cmd = Command::new("sudo");
        cmd.arg("rm");
    }
    cmd.args(args);
    let output = cmd.output().unwrap();
    if output.status.success() {
        // print output
        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    } else {
        Err(String::from_utf8(output.stderr).unwrap())
    }
}

// whoami
fn whoami() -> Result<String, String> {
    let output = Command::new("whoami").output().unwrap();
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout)
            .to_string()
            .replace("\r", "")
            .replace("\n", ""))
    } else {
        Err(String::from_utf8(output.stderr).unwrap())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let mut is_sudo = false;
    if let Ok(user_name) = whoami() {
        if user_name == "root" {
            is_sudo = true;
        }
    }
    if args.len() == 1 || args.iter().any(|&s| s == "-h" || s == "--help") {
        match rm(is_sudo, &["-h"]) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
            }
        }
        return;
    }
    println!("Are you sure you want to run this command?");
    if is_sudo {
        println!("Command: sudo rm {}", args[1..].join(" "));
    } else {
        println!("Command: rm {}", args[1..].join(" "));
    }
    let confirm_code = random_number();
    println!("Code: {}", confirm_code);
    let mut user_input = String::new();
    print!("please enter confirm code: ");
    let _ = stdout().flush();
    stdin().read_line(&mut user_input).unwrap();
    if user_input.trim() == confirm_code.to_string() {
        match rm(is_sudo, &args[1..]) {
            Ok(_) => println!("Removed successfully"),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("error: confirm code does not match");
    }
}

// generate a 4 number random number
fn random_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1000..9999)
}
