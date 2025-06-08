mod services;

use services::get_service_name;
use std::io;
use std::io::{stdin, stdout, Read, Write};
use std::net::SocketAddr;
use std::net::TcpStream;
use std::process;
use std::{thread, time::Duration};

fn main() {
    print_menu_items();

    let mut input_string: String = String::new();

    loop {
        input_string.clear();
        match io::stdin().read_line(&mut input_string) {
            Ok(_) => match input_string.as_str().trim() {
                "1" => scanner(),
                "2" => profile_scan(),
                "3" => end_program(),
                _ => menu_fallback(),
            },
            Err(_) => menu_fallback(),
        }
    }
}

fn scanner() {
    clear_screen();

    let mut ip_input = String::new();

    println!("Please enter IP address");
    ip_input.clear();
    match io::stdin().read_line(&mut ip_input) {
        Ok(_) => println!("Selected IP address {}", ip_input),
        Err(_) => {
            println!("Failed to read IP address");
            menu_fallback();
            return;
        }
    }

    let ip_input = ip_input.trim();

    println!("Scan multiple ports? (y/n)");
    let mut multi_choice = String::new();
    match io::stdin().read_line(&mut multi_choice) {
        Ok(_) => {}
        Err(_) => {
            println!("Failed to read choice");
            menu_fallback();
            return;
        }
    }

    if multi_choice.trim().to_lowercase() == "y" {
        let mut start_port_input = String::new();
        let mut end_port_input = String::new();

        println!("Please enter START port number");
        start_port_input.clear();
        match io::stdin().read_line(&mut start_port_input) {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to read start port");
                menu_fallback();
                return;
            }
        }

        println!("Please enter END port number");
        end_port_input.clear();
        match io::stdin().read_line(&mut end_port_input) {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to read end port");
                menu_fallback();
                return;
            }
        }

        let start_port = match start_port_input.trim().parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                println!("Invalid start port");
                menu_fallback();
                return;
            }
        };

        let end_port = match end_port_input.trim().parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                println!("Invalid end port");
                menu_fallback();
                return;
            }
        };

        if start_port > end_port {
            println!("Start port must be less than end port");
            menu_fallback();
            return;
        }

        println!(
            "\nScanning ports {}-{} on {}",
            start_port, end_port, ip_input
        );
        println!("This might take a while...\n");

        let mut open_ports = Vec::new();
        let total_ports = end_port - start_port + 1;
        let mut scanned_count = 0;

        for port in start_port..=end_port {
            scanned_count += 1;

            let percentage = (scanned_count as f32 / total_ports as f32 * 100.0) as u32;
            print!(
                "\rScanning port {} [{}/{}] {}% ",
                port, scanned_count, total_ports, percentage
            );
            print_progress_bar(percentage);
            io::stdout().flush().unwrap();

            let socket_addr = match format!("{}:{}", ip_input, port).parse::<SocketAddr>() {
                Ok(addr) => addr,
                Err(_) => {
                    continue;
                }
            };

            match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(1)) {
                Ok(_) => {
                    print!("\r");
                    print!("{}", " ".repeat(60));
                    let service_name = get_service_name(port);
                    print!("\rPort {} ({}) is OPEN\n", port, service_name);
                    open_ports.push(port);

                    print!(
                        "Scanning port {} [{}/{}] {}% ",
                        port, scanned_count, total_ports, percentage
                    );
                    print_progress_bar(percentage);
                    io::stdout().flush().unwrap();
                }
                Err(_) => {
                    // Don't print closed ports to reduce noise
                }
            }
        }

        print!("\r");
        print!("{}", " ".repeat(60));
        print!("\r");

        println!("\nScan complete!");
        println!("Found {} open ports", open_ports.len());
        if !open_ports.is_empty() {
            println!("\nOpen ports:");
            for port in open_ports.iter() {
                let service_name = get_service_name(*port);
                println!("  Port {:<6} {:<15} OPEN", port, service_name);
            }
        }
    } else {
        let mut port_input = String::new();

        println!("Please enter Port number");
        port_input.clear();
        match io::stdin().read_line(&mut port_input) {
            Ok(_) => println!("Selected Port {}", port_input),
            Err(_) => {
                println!("Failed to read port");
                menu_fallback();
                return;
            }
        }

        let port_input_formatted = match port_input.trim().parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                println!("Failed to read port");
                menu_fallback();
                return;
            }
        };

        println!(
            "Scanning Port {} on IP address {}",
            port_input_formatted, ip_input
        );

        print!("Scanning... ");
        for _ in 0..3 {
            print!(".");
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(300));
        }

        let socket_addr =
            match format!("{}:{}", ip_input, port_input_formatted).parse::<SocketAddr>() {
                Ok(addr) => addr,
                Err(_) => {
                    println!("\nInvalid address format");
                    return;
                }
            };

        match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(3)) {
            Ok(_) => {
                let service_name = get_service_name(port_input_formatted);
                println!(" OPEN ({})", service_name);
            }
            Err(_) => println!(" CLOSED"),
        }
    }

    press_enter_to_continue();
}

#[derive(Debug)]
enum ScanProfile {
    Quick,
    Web,
    Database,
    Full,
}

impl ScanProfile {
    fn get_ports(&self) -> Vec<u16> {
        match self {
            ScanProfile::Quick => vec![
                21, 22, 23, 25, 53, 80, 110, 143, 443, 445, 993, 995, 1723, 3306, 3389, 5900, 8080,
            ],
            ScanProfile::Web => vec![80, 443, 8080, 8443, 8000, 8008, 8088, 3128, 8888, 9000],
            ScanProfile::Database => {
                vec![3306, 5432, 1433, 1521, 27017, 6379, 9200, 5984, 7000, 8086]
            }
            ScanProfile::Full => vec![
                21, 22, 23, 25, 53, 80, 110, 111, 135, 139, 143, 443, 445, 993, 995, 1433, 1521,
                1723, 3306, 3389, 5432, 5900, 5984, 6379, 7000, 8080, 8086, 8443, 9200, 27017,
            ],
        }
    }

    fn get_name(&self) -> &str {
        match self {
            ScanProfile::Quick => "Quick Scan",
            ScanProfile::Web => "Web Services",
            ScanProfile::Database => "Database Services",
            ScanProfile::Full => "Full Common Ports",
        }
    }
}

fn profile_scan() {
    clear_screen();
    println!("=== Profile-Based Port Scanner ===\n");

    let mut ip_input = String::new();
    println!("Please enter IP address:");
    match io::stdin().read_line(&mut ip_input) {
        Ok(_) => {}
        Err(_) => {
            println!("Failed to read IP address");
            menu_fallback();
            return;
        }
    }
    let ip_input = ip_input.trim();

    println!("\nSelect scan profile:");
    println!("1. Quick Scan (17 ports)");
    println!("2. Web Services (10 ports)");
    println!("3. Database Services (10 ports)");
    println!("4. Full Scan (30 ports)");
    print!("\nYour choice: ");
    io::stdout().flush().unwrap();

    let mut profile_choice = String::new();
    match io::stdin().read_line(&mut profile_choice) {
        Ok(_) => {}
        Err(_) => {
            println!("Failed to read choice");
            menu_fallback();
            return;
        }
    }

    let profile = match profile_choice.trim() {
        "1" => ScanProfile::Quick,
        "2" => ScanProfile::Web,
        "3" => ScanProfile::Database,
        "4" => ScanProfile::Full,
        _ => {
            println!("Invalid choice");
            menu_fallback();
            return;
        }
    };

    let ports_to_scan = profile.get_ports();
    let total_ports = ports_to_scan.len();

    println!(
        "\n{} - Scanning {} ports on {}\n",
        profile.get_name(),
        total_ports,
        ip_input
    );

    let mut open_ports = Vec::new();

    for (index, port) in ports_to_scan.iter().enumerate() {
        let percentage = ((index + 1) as f32 / total_ports as f32 * 100.0) as u32;
        let service_name = get_service_name(*port);

        print!("\rScanning {} ({})... ", service_name, port);
        io::stdout().flush().unwrap();

        let socket_addr = match format!("{}:{}", ip_input, port).parse::<SocketAddr>() {
            Ok(addr) => addr,
            Err(_) => continue,
        };

        match TcpStream::connect_timeout(&socket_addr, Duration::from_millis(500)) {
            Ok(_) => {
                print!("\r\x1b[2K");
                println!("✓ {} ({}) - OPEN", service_name, port);
                open_ports.push(*port);

                print!("Progress: [{}/{}] {}% ", index + 1, total_ports, percentage);
                print_progress_bar(percentage);
                io::stdout().flush().unwrap();
            }
            Err(_) => {
                print!(
                    "\rProgress: [{}/{}] {}% ",
                    index + 1,
                    total_ports,
                    percentage
                );
                print_progress_bar(percentage);
                io::stdout().flush().unwrap();
            }
        }
    }

    print!("\r\x1b[2K");

    println!("\n{} Scan Complete!", profile.get_name());
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!(
        "Found {} open ports out of {} scanned",
        open_ports.len(),
        total_ports
    );

    if !open_ports.is_empty() {
        println!("\nSummary of open services:");
        for port in &open_ports {
            let service = get_service_name(*port);
            println!("  • {:<15} on port {}", service, port);
        }
    } else {
        println!("\nNo open ports found.");
    }

    press_enter_to_continue();
}

fn print_progress_bar(percentage: u32) {
    let bar_width: usize = 20;
    let filled = (bar_width * percentage as usize / 100);
    let empty = bar_width.saturating_sub(filled);

    print!("[");
    print!("{}", "=".repeat(filled));
    if filled < bar_width {
        print!(">");
        if empty > 1 {
            print!("{}", " ".repeat(empty - 1));
        }
    }
    print!("]");
}

enum MenuItem {
    CustomScan,
    ProfileScan,
    Exit,
}

impl MenuItem {
    fn get_description(&self) -> String {
        match self {
            MenuItem::CustomScan => String::from("1. Custom Port Scan"),
            MenuItem::ProfileScan => String::from("2. Profile Scan"),
            MenuItem::Exit => String::from("3. Exit"),
        }
    }
}

struct ScannerBasicInfo {
    name: String,
    version: f32,
}

fn print_menu_items() {
    clear_screen();

    let my_scanner: ScannerBasicInfo = ScannerBasicInfo {
        name: String::from("Vonogs Scanner"),
        version: 0.3,
    };
    println!("{} v{}", my_scanner.name, my_scanner.version);
    println!("====================");

    let menu: [MenuItem; 3] = [MenuItem::CustomScan, MenuItem::ProfileScan, MenuItem::Exit];

    for item in &menu {
        println!("{}", item.get_description());
    }

    println!("====================");
    print!("Select an option: ");
    io::stdout().flush().unwrap();
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "cls"])
            .status();
    } else {
        let _ = std::process::Command::new("clear").status();
    }
}

fn menu_fallback() {
    clear_screen();
    println!("Please select option from the menu.");
    thread::sleep(Duration::from_millis(2000));
    print_menu_items();
}

fn end_program() {
    println!("\nThank you for using Vonogs Scanner!");
    println!("Goodbye!");
    thread::sleep(Duration::from_millis(1000));
    process::exit(0);
}

fn press_enter_with_message(message: &str) {
    let mut stdout = stdout();
    write!(stdout, "\n{}", message).unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();

    clear_screen();
    print_menu_items();
}

fn press_enter_to_continue() {
    press_enter_with_message("Press Enter to continue...");
}
