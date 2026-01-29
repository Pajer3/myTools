use chrono::Utc;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Output};

fn main() -> io::Result<()> {
    let out1 = Command::new("hyprctl")
        .arg("seterror")
        .arg("")
        .output();
    log_command_result("hyprctl_seterror", &out1)?;

    let _ = Command::new("setsid")
        .arg("alacritty")
        .arg("-e")
        .arg("bash")
        .arg("-c")
        .arg("sudo pacman -Syu; echo 'Update complete. Press Enter to exit...'; read")
        .spawn();

    let tauri_bin = "/home/newhyde/myTools/apps/desktop/src-tauri/target/release/desktop";
    let out2 = Command::new(tauri_bin).output();
    log_command_result("tauri_release_run", &out2)?;

    Ok(())
}

fn log_command_result(label: &str, result: &io::Result<Output>) -> io::Result<()> {
    let log_dir = Path::new("/home/newhyde/myTools/agent_logs");
    fs::create_dir_all(log_dir)?;

    clean_old_logs(log_dir, label)?;

    let stamp = Utc::now().format("%Y-%m-%d_%H-%M-%S");
    let log_path = log_dir.join(format!("agent_errors_{label}_{stamp}.txt"));
    let mut file = File::create(log_path)?;

    match result {
        Ok(output) => {
            writeln!(file, "[label] {label}")?;
            writeln!(file, "[status] {}", output.status)?;
            writeln!(file, "\n[stdout]\n{}", String::from_utf8_lossy(&output.stdout))?;
            writeln!(file, "\n[stderr]\n{}", String::from_utf8_lossy(&output.stderr))?;
        }
        Err(e) => {
            writeln!(file, "[label] {label}")?;
            writeln!(file, "[error] failed to run command: {e}")?;
        }
    }

    Ok(())
}

fn clean_old_logs(dir: &Path, label: &str) -> io::Result<()> {
    let prefix = format!("agent_errors_{label}_");

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with(&prefix) && name.ends_with(".txt") {
                    let _ = fs::remove_file(&path); // ignore per-file delete errors
                }
            }
        }
    }

    Ok(())
}
