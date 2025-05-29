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
                "2" => quick_scan(),
                "3" => common_ports_scan(),
                "4" => end_program(),
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
                    print!("\rPort {} is OPEN\n", port);
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
            print!("Open ports: ");
            for (i, port) in open_ports.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}", port);
            }
            println!();
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
            Ok(_) => println!(" OPEN"),
            Err(_) => println!(" CLOSED"),
        }
    }

    press_enter_to_continue();
}

fn quick_scan() {
    clear_screen();
    println!("=== Quick Scan (Top 20 ports) ===\n");

    let mut ip_input = String::new();
    println!("Please enter IP address");
    match io::stdin().read_line(&mut ip_input) {
        Ok(_) => {}
        Err(_) => {
            println!("Failed to read IP address");
            menu_fallback();
            return;
        }
    }

    let ip_input = ip_input.trim();

    let quick_ports = [
        21, 22, 23, 25, 53, 80, 110, 111, 135, 139, 143, 443, 445, 993, 995, 1723, 3306, 3389,
        5900, 8080,
    ];

    println!("Scanning top {} ports on {}\n", quick_ports.len(), ip_input);

    let mut open_ports = Vec::new();
    let total_ports = quick_ports.len();

    for (index, port) in quick_ports.iter().enumerate() {
        let percentage = ((index + 1) as f32 / total_ports as f32 * 100.0) as u32;
        print!(
            "\rProgress: [{}/{}] {}% ",
            index + 1,
            total_ports,
            percentage
        );
        print_progress_bar(percentage);
        io::stdout().flush().unwrap();

        let socket_addr = match format!("{}:{}", ip_input, port).parse::<SocketAddr>() {
            Ok(addr) => addr,
            Err(_) => continue,
        };

        match TcpStream::connect_timeout(&socket_addr, Duration::from_millis(500)) {
            Ok(_) => {
                open_ports.push(*port);
            }
            Err(_) => {
                // Don't print closed ports to reduce noise
            }
        }
    }

    print!("\r");
    print!("{}", " ".repeat(60));
    print!("\r");

    println!("Quick scan complete!");
    println!(
        "Found {} open ports out of {} scanned",
        open_ports.len(),
        quick_ports.len()
    );

    if !open_ports.is_empty() {
        print!("Open ports: ");
        for (i, port) in open_ports.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", port);
        }
        println!();
    }

    if open_ports.is_empty() {
        press_enter_with_message("No open ports found. Press Enter to return to menu...");
    } else {
        press_enter_with_message("Scan successful! Press Enter to return to menu...");
    }
}

fn common_ports_scan() {
    clear_screen();
    println!("=== Common Services Scan ===\n");

    let mut ip_input = String::new();
    println!("Please enter IP address");
    match io::stdin().read_line(&mut ip_input) {
        Ok(_) => {}
        Err(_) => {
            println!("Failed to read IP address");
            menu_fallback();
            return;
        }
    }

    let ip_input = ip_input.trim();

    let services = [
        (21, "FTP"),
        (22, "SSH"),
        (23, "Telnet"),
        (25, "SMTP"),
        (53, "DNS"),
        (80, "HTTP"),
        (110, "POP3"),
        (143, "IMAP"),
        (443, "HTTPS"),
        (445, "SMB"),
        (3306, "MySQL"),
        (3389, "RDP"),
        (5432, "PostgreSQL"),
        (5900, "VNC"),
        (8080, "HTTP-Alt"),
    ];

    println!(
        "Scanning {} common service ports on {}\n",
        services.len(),
        ip_input
    );

    let mut found_services = Vec::new();

    for (port, service) in services.iter() {
        let socket_addr = match format!("{}:{}", ip_input, port).parse::<SocketAddr>() {
            Ok(addr) => addr,
            Err(_) => continue,
        };

        print!("Checking {} ({})... ", service, port);
        io::stdout().flush().unwrap();

        match TcpStream::connect_timeout(&socket_addr, Duration::from_millis(500)) {
            Ok(_) => {
                println!("OPEN ✓");
                found_services.push((*port, *service));
            }
            Err(_) => {
                println!("closed");
            }
        }
    }

    println!("\nScan complete!");
    if !found_services.is_empty() {
        println!("\nDiscovered services:");
        for (port, service) in found_services.iter() {
            println!("  {} ({})", service, port);
        }
        press_enter_with_message("\nServices found! Press Enter to save results and continue...");
    } else {
        println!("No common services found.");
        press_enter_to_continue();
    }
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
    QuickScan,
    CommonPorts,
    Exit,
}

impl MenuItem {
    fn get_description(&self) -> String {
        match self {
            MenuItem::CustomScan => String::from("1. Custom Port Scan"),
            MenuItem::QuickScan => String::from("2. Quick Scan (Top 20)"),
            MenuItem::CommonPorts => String::from("3. Common Services"),
            MenuItem::Exit => String::from("4. Exit"),
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
        version: 0.2,
    };
    println!("{} v{}", my_scanner.name, my_scanner.version);
    println!("====================");

    let menu: [MenuItem; 4] = [
        MenuItem::CustomScan,
        MenuItem::QuickScan,
        MenuItem::CommonPorts,
        MenuItem::Exit,
    ];

    for item in &menu {
        println!("{}", item.get_description());
    }

    println!("====================");
    print!("Select an option: ");
    io::stdout().flush().unwrap();
}

fn clear_screen() {
    if let Err(_) = std::process::Command::new("clear").status() {
        println!("\n\n\n\n");
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

fn press_enter() {
    press_enter_with_message("Press Enter to continue...");
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
