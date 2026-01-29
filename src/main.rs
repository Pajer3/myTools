use std::process::Command;
use std::fs::File;
use chrono::prelude::*;

fn main() {
    let rf_shader_err = Command::new("hyprctl")
        .arg("seterror")
        .arg("")
        .output()
        .output_agent_errors();

    
}


fn output_agent_errors(output: std::process::Output) -> File {
    let date = Utc::now();
    let file = File::create(format!("agent_errors_{}.txt", date)).expect("failed to create file");
    file
}