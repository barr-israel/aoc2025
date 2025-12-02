use std::process::{Command, Stdio};

use crate::template::Day;

pub fn run(day: Day, part: u8) {
    let mut cmd_args = vec![
        "run".to_string(),
        "--bin".to_string(),
        day.to_string(),
        "--release".to_string(),
    ];

    cmd_args.push("--".to_string());
    cmd_args.push("--tui".to_string());
    cmd_args.push("--part".to_string());
    cmd_args.push(part.to_string());

    let mut cmd = Command::new("cargo")
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}
