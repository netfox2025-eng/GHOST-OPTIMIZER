use sysinfo::{ProcessExt, System, SystemExt};
use std::time::Duration;
use tokio::time::sleep;
use reqwest::Client;
use serde_json::json;

// Build ke waqt GitHub Secret se token aur URL automatic yahan inject ho jayenge
const HF_TOKEN: &str = env!("HF_ACCESS_TOKEN");
const HF_SPACE_URL: &str = env!("HF_SPACE_URL");

#[tokio::main]
async fn main() {
    println!("==================================================");
    println!("===        GHOST OPTIMIZER CLI (v0.1.0)        ===");
    println!("==================================================");
    println!("Initializing secure infrastructure optimizer...\n");

    let mut sys = System::new_all();
    let client = Client::new();

    loop {
        sys.refresh_all();
        println!("--------------------------------------------------");
        println!("Checking for CPU-heavy and unoptimized processes...");
        println!("--------------------------------------------------");

        let mut heavy_processes = false;

        for (pid, process) in sys.processes() {
            let cpu_usage = process.cpu_usage();
            
            if cpu_usage > 80.0 {
                heavy_processes = true;
                let proc_name = process.name().to_string_lossy().to_string(); 
                
                println!(
                    "[⚠️ ALERT] Process ID: {} | Name: {} | CPU Usage: {:.2}%",
                    pid, proc_name, cpu_usage
                );
                
                optimize_process(&client, pid.to_string(), proc_name, cpu_usage).await;
            }
        }

        if !heavy_processes {
            println!("[🟢 HEALTHY] All enterprise infrastructure processes running under optimal limits.");
        }

        sleep(Duration::from_secs(5)).await;
    }
}

async fn optimize_process(client: &Client, pid: String, name: String, cpu: f32) {
    println!("[💡 GHOST ACTION] Fetching secure brain analysis for '{}' (PID: {})...", name, pid);
    
    // Authorization Header format: "Bearer hf_..."
    let auth_header = format!("Bearer {}", HF_TOKEN);

    let response = client.post(HF_SPACE_URL)
        .header("Authorization", auth_header)
        .json(&json!({
            "process_name": name,
            "cpu_usage": cpu,
            "secret_key": "SUPER_SECRET_GHOST_KEY_123"
        }))
        .send()
        .await;

    if let Ok(res) = response {
        if let Ok(json_data) = res.json::<serde_json::Value>().await {
            println!("   -> {}", json_data["recommendation"].as_str().unwrap_or("Optimal setup"));
        }
    } else {
        println!("   -> [❌ ERROR] Could not connect to Ghost Private Brain Server.");
    }
}
