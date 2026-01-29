use std::process::Command;
use std::collections::HashMap;

struct Workflow {
    name: &str,
}

struct Flow {
    active: bool,
    commands: Command,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn execute_workflow(name: &str) {
    println!("Executing workflow: {}", name);
    let execute_workflow = match name {
        "School Enviroment" => "",
        "Work Enviroment" => "",
        "Hack Enviroment" => "",
        _ => "",
    };
}

#[tauri::command]
async fn open_file_manager() {
    // You can use the opener crate internally or just standard Command
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        let _ = Command::new("xdg-open").arg(".").spawn(); 
    }
     #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let _ = Command::new("explorer").arg(".").spawn();
    }
     #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let _ = Command::new("open").arg(".").spawn();
    }
}

async fn school_enviroment() {
    let map: HashMap<Workflow, Flow> = HashMap::new();
    let commands: Vec<Command> = vec![
        Command::new("sudo systemctl start").arg("docker"),
        Command::new("")
    ];
    map.insert(
        Workflow { name: "School Enviroment"},
        Flow { active: false, commands: Command::new("")}
    )
    
}

async fn work_enviroment() {
    
}

async fn hack_enviroment() {
    
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, execute_workflow, open_file_manager])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
