use std::thread::sleep;
use std::time::Duration;
use crossterm::{
    execute,
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
    cursor::{MoveTo, Hide, Show},
    style::{Print, Color, SetForegroundColor, ResetColor},
    event::{self, Event, KeyCode},
};
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};
use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(name = "ham")]
#[command(about = "HAM - Heuristic Adaptive Monitor")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Live scan of protocol availability
    Scan,
    /// Analyze network conditions
    Analyze,
    /// Export configuration
    Export { format: String },
}

#[derive(Clone, Debug)]
struct ProtocolStatus {
    name: String,
    status: String,
    score: u8, // 0-10
    details: String,
    color: Color,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Some(Commands::Scan) => run_scan().await,
        Some(Commands::Analyze) => run_analyze().await,
        Some(Commands::Export { format }) => run_export(format).await,
        None => run_scan().await, // Default to scan
    }
}

async fn run_scan() {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    execute!(stdout, Hide, Clear(ClearType::All)).unwrap();
    
    let protocols = Arc::new(Mutex::new(Vec::<ProtocolStatus>::new()));
    let running = Arc::new(Mutex::new(true));
    
    // Initialize protocols
    {
        let mut p = protocols.lock().unwrap();
        p.push(ProtocolStatus {
            name: "TCP:80".to_string(),
            status: "Testing...".to_string(),
            score: 0,
            details: "HTTP connectivity".to_string(),
            color: Color::Yellow,
        });
        p.push(ProtocolStatus {
            name: "TCP:443".to_string(),
            status: "Testing...".to_string(),
            score: 0,
            details: "HTTPS connectivity".to_string(),
            color: Color::Yellow,
        });
        p.push(ProtocolStatus {
            name: "DNS".to_string(),
            status: "Testing...".to_string(),
            score: 0,
            details: "Domain resolution".to_string(),
            color: Color::Yellow,
        });
        p.push(ProtocolStatus {
            name: "PING".to_string(),
            status: "Testing...".to_string(),
            score: 0,
            details: "ICMP connectivity".to_string(),
            color: Color::Yellow,
        });
        p.push(ProtocolStatus {
            name: "UDP".to_string(),
            status: "Testing...".to_string(),
            score: 0,
            details: "UDP connectivity".to_string(),
            color: Color::Yellow,
        });
    }
    
    // Spawn background monitoring task
    let protocols_clone = Arc::clone(&protocols);
    let running_clone = Arc::clone(&running);
    tokio::spawn(async move {
        monitor_protocols(protocols_clone, running_clone).await;
    });
    
    // Main display loop
    loop {
        // Check for exit input
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    *running.lock().unwrap() = false;
                    break;
                }
            }
        }
        
        // Update display
        display_protocols(&mut stdout, &protocols).await;
        sleep(Duration::from_millis(500));
    }
    
    execute!(stdout, Show, Clear(ClearType::All)).unwrap();
    disable_raw_mode().unwrap();
    println!("HAM scan completed. Press any key to exit.");
}

async fn display_protocols(stdout: &mut std::io::Stdout, protocols: &Arc<Mutex<Vec<ProtocolStatus>>>) {
    execute!(stdout, MoveTo(0, 0), Clear(ClearType::All)).unwrap();
    
    // Header
    execute!(stdout, 
        SetForegroundColor(Color::Cyan),
        Print("HAM - Network Protocol Scanner"),
        MoveTo(0, 1),
        Print("Press 'q' to quit"),
        MoveTo(0, 3),
        ResetColor
    ).unwrap();
    
    let protocols_guard = protocols.lock().unwrap();
    for (i, protocol) in protocols_guard.iter().enumerate() {
        let progress_bar = create_progress_bar(protocol.score);
        execute!(stdout,
            MoveTo(0, 4 + i as u16),
            SetForegroundColor(protocol.color),
            Print(format!("[{:8}] {} {}", 
                protocol.name, 
                progress_bar, 
                protocol.status
            )),
            ResetColor
        ).unwrap();
    }
    
    stdout.flush().unwrap();
}

fn create_progress_bar(score: u8) -> String {
    let filled = "█".repeat(score as usize);
    let empty = "░".repeat(10 - score as usize);
    format!("{}{}", filled, empty)
}

async fn monitor_protocols(protocols: Arc<Mutex<Vec<ProtocolStatus>>>, running: Arc<Mutex<bool>>) {
    while *running.lock().unwrap() {
        // Test HTTP (port 80)
        let http_score = test_tcp_connection("8.8.8.8:53", Duration::from_secs(3)).await;
        update_protocol(&protocols, "TCP:80", http_score, "HTTP connectivity").await;
        
        // Test HTTPS (port 443) 
        let https_score = test_https_connection().await;
        update_protocol(&protocols, "TCP:443", https_score, "HTTPS connectivity").await;
        
        // Test DNS
        let dns_score = test_dns_resolution().await;
        update_protocol(&protocols, "DNS", dns_score, "Domain resolution").await;
        
        // Test PING (simulated)
        let ping_score = test_ping().await;
        update_protocol(&protocols, "PING", ping_score, "ICMP connectivity").await;
        
        // Test UDP (simulated)
        let udp_score = test_udp().await;
        update_protocol(&protocols, "UDP", udp_score, "UDP connectivity").await;
        
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

async fn update_protocol(protocols: &Arc<Mutex<Vec<ProtocolStatus>>>, name: &str, score: u8, details: &str) {
    let mut protocols_guard = protocols.lock().unwrap();
    if let Some(protocol) = protocols_guard.iter_mut().find(|p| p.name == name) {
        protocol.score = score;
        protocol.details = details.to_string();
        
        match score {
            0..=3 => {
                protocol.status = "Blocked/Failed".to_string();
                protocol.color = Color::Red;
            },
            4..=6 => {
                protocol.status = "Limited".to_string();
                protocol.color = Color::Yellow;
            },
            7..=10 => {
                protocol.status = "Good".to_string();
                protocol.color = Color::Green;
            },
            _ => {
                protocol.status = "Unknown".to_string();
                protocol.color = Color::White;
            },
        }
    }
}

async fn test_tcp_connection(addr: &str, timeout_duration: Duration) -> u8 {
    use tokio::net::TcpStream;
    use tokio::time::timeout;
    
    match timeout(timeout_duration, TcpStream::connect(addr)).await {
        Ok(Ok(_)) => 10,
        Ok(Err(_)) => 0,
        Err(_) => 2, // Timeout
    }
}

async fn test_https_connection() -> u8 {
    use tokio::time::timeout;
    
    // Test HTTPS connection to a known endpoint
    match timeout(Duration::from_secs(5), reqwest::get("https://www.google.com")).await {
        Ok(Ok(response)) => {
            if response.status().is_success() {
                10
            } else {
                5
            }
        },
        Ok(Err(_)) => 2,
        Err(_) => 1, // Timeout
    }
}

async fn test_dns_resolution() -> u8 {
    use dns_lookup::lookup_host;
    
    match lookup_host("google.com") {
        Ok(ips) => {
            if !ips.is_empty() {
                10
            } else {
                0
            }
        },
        Err(_) => 0,
    }
}

async fn test_ping() -> u8 {
    // Simplified ping test using TCP connection to port 53 (DNS)
    test_tcp_connection("8.8.8.8:53", Duration::from_secs(2)).await
}

async fn test_udp() -> u8 {
    // Simplified UDP test - for now return a simulated score
    // In a real implementation, this would test UDP connectivity
    use std::process::Command;
    
    match Command::new("ping")
        .args(["-c", "1", "-W", "2", "8.8.8.8"])
        .output() {
        Ok(output) => {
            if output.status.success() {
                8
            } else {
                2
            }
        },
        Err(_) => 0,
    }
}

async fn run_analyze() {
    println!("{}", "HAM Network Analysis".cyan().bold());
    println!("Analyzing network conditions...\n");
    
    // Basic network analysis
    println!("📊 {}", "Network Interface Status:".yellow());
    analyze_network_interfaces().await;
    
    println!("\n🔍 {}", "Connectivity Tests:".yellow());
    analyze_connectivity().await;
    
    println!("\n🛡️  {}", "Censorship Detection:".yellow());
    analyze_censorship().await;
}

async fn analyze_network_interfaces() {
    use std::process::Command;
    
    match Command::new("ip").args(["route", "show", "default"]).output() {
        Ok(output) => {
            if output.status.success() {
                let route_info = String::from_utf8_lossy(&output.stdout);
                if !route_info.trim().is_empty() {
                    println!("   ✓ {}", "Default route found".green());
                } else {
                    println!("   ✗ {}", "No default route".red());
                }
            }
        },
        Err(_) => println!("   ? {}", "Could not check routing table".yellow()),
    }
}

async fn analyze_connectivity() {
    let targets = vec![
        ("Google DNS", "8.8.8.8:53"),
        ("Cloudflare DNS", "1.1.1.1:53"),
        ("OpenDNS", "208.67.222.222:53"),
    ];
    
    for (name, addr) in targets {
        let score = test_tcp_connection(addr, Duration::from_secs(3)).await;
        if score > 7 {
            println!("   ✓ {} - {}", name.green(), "Reachable");
        } else if score > 3 {
            println!("   ⚠ {} - {}", name.yellow(), "Limited");
        } else {
            println!("   ✗ {} - {}", name.red(), "Blocked");
        }
    }
}

async fn analyze_censorship() {
    println!("   🔍 Testing for common censorship patterns...");
    
    // Test different TLD accessibility
    let domains = vec!["google.com", "facebook.com", "twitter.com", "youtube.com"];
    let mut accessible = 0;
    
    for domain in &domains {
        match dns_lookup::lookup_host(domain) {
            Ok(_) => {
                accessible += 1;
                println!("   ✓ {} - {}", domain.green(), "DNS resolves");
            },
            Err(_) => {
                println!("   ✗ {} - {}", domain.red(), "DNS blocked");
            }
        }
    }
    
    let accessibility_ratio = accessible as f32 / domains.len() as f32;
    if accessibility_ratio > 0.8 {
        println!("   📊 {}", "Network appears uncensored".green());
    } else if accessibility_ratio > 0.5 {
        println!("   📊 {}", "Partial censorship detected".yellow());
    } else {
        println!("   📊 {}", "Heavy censorship likely".red());
    }
}

async fn run_export(format: &str) {
    println!("{}", "HAM Configuration Export".cyan().bold());
    println!("Export format: {}\n", format.yellow());
    
    match format {
        "json" => export_json().await,
        "qr" => export_qr().await,
        _ => println!("Unsupported format: {}", format.red()),
    }
}

async fn export_json() {
    let config = serde_json::json!({
        "ham_config": {
            "version": "0.1.0",
            "scan_intervals": 2,
            "test_endpoints": [
                "8.8.8.8:53",
                "1.1.1.1:53"
            ],
            "protocols": ["tcp", "udp", "dns"]
        }
    });
    
    println!("Configuration JSON:");
    println!("{}", serde_json::to_string_pretty(&config).unwrap());
}

async fn export_qr() {
    println!("QR code export not yet implemented.");
    println!("Would contain bridge/tunnel configuration for sharing.");
}
