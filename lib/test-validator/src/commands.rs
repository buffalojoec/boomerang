use std::process::{Command, Stdio};

pub fn run_command(command: &str) {
    let status = Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .expect("failed to execute process");
    assert!(status.success());
}

pub fn run_command_with_num_retries(command: &str, num_retries: u8) {
    let mut retries = 0;
    loop {
        let status = Command::new("sh")
            .arg("-c")
            .arg(command)
            .status()
            .expect("failed to execute process");
        if status.success() {
            break;
        }
        retries += 1;
        if retries > num_retries {
            panic!("failed to execute process");
        }
    }
}

pub fn run_command_detached(command: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to execute process");
}
