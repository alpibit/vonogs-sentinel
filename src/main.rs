mod services;
mod validation;

use services::get_service_name;
use std::fs::{self, File};
use std::io;
use std::io::{stdin, stdout, BufWriter, Write};
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::path::Path;
use std::process;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use validation::{is_valid_ip, is_valid_port};

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const BOLD: &str = "\x1b[1m";

fn main() {
    create_logs_directory();
    print_menu_items();

    loop {
        match read_input("") {
            Ok(input) => match input.as_str() {
                "1" => scanner(),
                "2" => profile_scan(),
                "3" => end_program(),
                _ => menu_fallback(),
            },
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => end_program(),
            Err(_) => menu_fallback(),
        }
    }
}

fn create_logs_directory() {
    if !Path::new("scan_logs").exists() {
        let _ = fs::create_dir("scan_logs");
    }
}

fn read_input(prompt: &str) -> io::Result<String> {
    if !prompt.is_empty() {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
    }

    let mut input = String::new();
    let bytes_read = io::stdin().read_line(&mut input)?;
    if bytes_read == 0 {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "end of input"));
    }
    Ok(input.trim().to_string())
}

fn read_u16(prompt: &str) -> io::Result<u16> {
    let input = read_input(prompt)?;
    match input.trim().parse::<u16>() {
        Ok(port) => Ok(port),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid number",
        )),
    }
}

fn read_yes_no(prompt: &str) -> io::Result<bool> {
    loop {
        let input = read_input(prompt)?;
        match input.to_lowercase().as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => {
                println!("{}Please enter y or n.{}", YELLOW, RESET);
            }
        }
    }
}

fn read_port_or_menu(label: &str) -> Option<u16> {
    match read_u16("") {
        Ok(port) if is_valid_port(port) => Some(port),
        Ok(_) => {
            println!("{}Invalid {}{}", RED, label, RESET);
            thread::sleep(Duration::from_millis(2000));
            menu_fallback();
            None
        }
        Err(e) => {
            if e.kind() == io::ErrorKind::InvalidInput {
                println!("{}Invalid {}{}", RED, label, RESET);
                thread::sleep(Duration::from_millis(2000));
            } else {
                println!("{}Failed to read {}{}", RED, label, RESET);
            }
            menu_fallback();
            None
        }
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn get_timestamp() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let secs = now.as_secs();

    let seconds = (secs % 60) as u32;
    let minutes = ((secs / 60) % 60) as u32;
    let hours = ((secs / 3600) % 24) as u32;

    let mut days = (secs / 86_400) as i64;

    let mut year: i32 = 1970;
    loop {
        let year_days = if is_leap_year(year) { 366 } else { 365 };
        if days >= year_days {
            days -= year_days;
            year += 1;
        } else {
            break;
        }
    }

    let leap = is_leap_year(year);
    let month_lengths = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];

    let mut month: u32 = 1;
    for &len in &month_lengths {
        if days >= len as i64 {
            days -= len as i64;
            month += 1;
        } else {
            break;
        }
    }

    let day = (days + 1) as u32;

    format!(
        "{:04}-{:02}-{:02}_{:02}-{:02}-{:02}",
        year, month, day, hours, minutes, seconds
    )
}

fn null_log_path() -> &'static str {
    if cfg!(target_os = "windows") {
        "NUL"
    } else {
        "/dev/null"
    }
}

fn create_log_file(scan_type: &str) -> (BufWriter<File>, String) {
    let timestamp = get_timestamp();
    let filename = format!("scan_logs/scan_{}_{}.log", timestamp, scan_type);
    match File::create(&filename) {
        Ok(f) => (BufWriter::new(f), filename),
        Err(_) => {
            println!("{}Warning: Could not create log file{}", YELLOW, RESET);
            let f = File::create(null_log_path()).unwrap();
            (BufWriter::new(f), String::from("(no log file created)"))
        }
    }
}

fn write_log_header<W: Write>(log_file: &mut W, scan_type: &str, target_ip: &str) {
    let timestamp = get_timestamp();
    let header = format!(
        "=================================\n\
         Vonogs Scanner Log\n\
         =================================\n\
         Scan Type: {}\n\
         Target: {}\n\
         Start Time: {}\n\
         =================================\n\n",
        scan_type, target_ip, timestamp
    );
    let _ = log_file.write_all(header.as_bytes());
}

fn write_log_entry<W: Write>(log_file: &mut W, message: &str) {
    let _ = log_file.write_all(format!("{}\n", message).as_bytes());
}

fn write_log_summary<W: Write>(
    log_file: &mut W,
    open_ports: &[u16],
    total_scanned: u32,
    elapsed_secs: f32,
) {
    let summary = format!(
        "\n=================================\n\
         Scan Summary\n\
         =================================\n\
         Total Ports Scanned: {}\n\
         Open Ports Found: {}\n\
         Scan Duration: {:.2} seconds\n",
        total_scanned,
        open_ports.len(),
        elapsed_secs
    );
    let _ = log_file.write_all(summary.as_bytes());

    if !open_ports.is_empty() {
        let _ = log_file.write_all(b"\nOpen Ports:\n");
        for port in open_ports {
            let service = get_service_name(*port);
            let entry = format!("  Port {}: {} (OPEN)\n", port, service);
            let _ = log_file.write_all(entry.as_bytes());
        }
    }

    let end_time = format!("\nEnd Time: {}\n", get_timestamp());
    let _ = log_file.write_all(end_time.as_bytes());
}

fn connect_timeout() -> Duration {
    std::env::var("VONOGS_TIMEOUT_MS")
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
        .map(Duration::from_millis)
        .unwrap_or(Duration::from_millis(700))
}

#[derive(Debug, Clone, Copy)]
enum PortStatus {
    Open,
    Closed,
    TimeoutFiltered,
    InvalidAddress,
}

fn scan_port_ip(resolved_ip: Option<IpAddr>, port: u16) -> PortStatus {
    let ip = match resolved_ip {
        Some(ip) => ip,
        None => return PortStatus::InvalidAddress,
    };

    let socket_addr = SocketAddr::new(ip, port);

    match TcpStream::connect_timeout(&socket_addr, connect_timeout()) {
        Ok(_) => PortStatus::Open,
        Err(e) => {
            if e.kind() == io::ErrorKind::TimedOut {
                PortStatus::TimeoutFiltered
            } else {
                PortStatus::Closed
            }
        }
    }
}

fn resolve_target_note(target: &str) -> (Option<IpAddr>, Option<String>) {
    if is_valid_ip(target) {
        return (target.trim().parse::<IpAddr>().ok(), None);
    }

    match (target, 80).to_socket_addrs() {
        Ok(iter) => {
            let mut selected_v4 = None;
            let mut last_v6 = None;

            for addr in iter {
                if addr.is_ipv4() {
                    selected_v4 = Some(addr.ip());
                    break;
                } else {
                    last_v6 = Some(addr.ip());
                }
            }

            if let Some(ip) = selected_v4.or(last_v6) {
                let note = format!("Resolved Target: {} -> {}", target, ip);
                println!(
                    "{}Resolved {}{}{} to {}{}{}",
                    YELLOW, CYAN, target, RESET, CYAN, ip, RESET
                );
                (Some(ip), Some(note))
            } else {
                let note = format!("Resolution failed for '{}'", target);
                println!(
                    "{}Note: '{}' could not be resolved{}",
                    YELLOW, target, RESET
                );
                thread::sleep(Duration::from_millis(500));
                (None, Some(note))
            }
        }
        Err(_) => {
            let note = format!("Resolution failed for '{}'", target);
            println!(
                "{}Note: '{}' could not be resolved{}",
                YELLOW, target, RESET
            );
            thread::sleep(Duration::from_millis(500));
            (None, Some(note))
        }
    }
}

fn scanner() {
    clear_screen();
    let scan_started = Instant::now();

    println!("Please enter IP address or hostname");
    let ip_input_raw = match read_input("") {
        Ok(input) => {
            println!("Selected target {}{}{}", CYAN, input.as_str(), RESET);
            input
        }
        Err(_) => {
            println!("{}Failed to read IP address{}", RED, RESET);
            menu_fallback();
            return;
        }
    };

    let ip_input = ip_input_raw.as_str();

    let (resolved_ip, resolution_note) = resolve_target_note(ip_input);

    println!("Scan multiple ports? (y/n)");
    let multi_choice = match read_yes_no("") {
        Ok(choice) => choice,
        Err(_) => {
            println!("{}Failed to read choice{}", RED, RESET);
            menu_fallback();
            return;
        }
    };

    if multi_choice {
        println!("Please enter START port number");
        let Some(start_port) = read_port_or_menu("start port") else {
            return;
        };

        println!("Please enter END port number");
        let Some(end_port) = read_port_or_menu("end port") else {
            return;
        };

        if start_port > end_port {
            println!("{}Start port must be less than end port{}", RED, RESET);
            menu_fallback();
            return;
        }

        let (mut log_file, log_path) = create_log_file("custom_range");
        write_log_header(&mut log_file, "Custom Range Scan", ip_input);
        if let Some(note) = &resolution_note {
            write_log_entry(&mut log_file, note);
        }
        write_log_entry(
            &mut log_file,
            &format!("Port Range: {}-{}", start_port, end_port),
        );

        println!(
            "\nScanning ports {}{}-{}{} on {}{}{}",
            YELLOW, start_port, end_port, RESET, CYAN, ip_input, RESET
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

            match scan_port_ip(resolved_ip, port) {
                PortStatus::Open => {
                    print!("\r");
                    print!("{}", " ".repeat(60));
                    let service_name = get_service_name(port);
                    print!(
                        "\rPort {}{}{} ({}{}{}) is {}{}OPEN{}\n",
                        YELLOW, port, RESET, CYAN, service_name, RESET, GREEN, BOLD, RESET
                    );
                    open_ports.push(port);
                    write_log_entry(
                        &mut log_file,
                        &format!("Port {}: {} - OPEN", port, service_name),
                    );

                    print!(
                        "Scanning port {} [{}/{}] {}% ",
                        port, scanned_count, total_ports, percentage
                    );
                    print_progress_bar(percentage);
                    io::stdout().flush().unwrap();
                }
                PortStatus::TimeoutFiltered => {
                    write_log_entry(&mut log_file, &format!("Port {}: TIMEOUT/FILTERED", port));
                }
                PortStatus::Closed => {
                    write_log_entry(&mut log_file, &format!("Port {}: CLOSED", port));
                }
                PortStatus::InvalidAddress => {
                    write_log_entry(&mut log_file, &format!("Port {}: Invalid address", port));
                    continue;
                }
            }
        }

        print!("\r");
        print!("{}", " ".repeat(60));
        print!("\r");

        let elapsed = scan_started.elapsed().as_secs_f32();

        println!("\n{}{}Scan complete!{}", GREEN, BOLD, RESET);
        println!("Found {}{}{} open ports", GREEN, open_ports.len(), RESET);
        println!("{}Scan took {:.2} seconds{}", CYAN, elapsed, RESET);
        if !open_ports.is_empty() {
            println!("\n{}Open ports{}:", YELLOW, RESET);
            for port in open_ports.iter() {
                let service_name = get_service_name(*port);
                println!(
                    "  Port {}{:<6}{} {}{:<15}{} {}OPEN{}",
                    YELLOW, port, RESET, CYAN, service_name, RESET, GREEN, RESET
                );
            }
        }

        write_log_summary(&mut log_file, &open_ports, total_ports as u32, elapsed);
        let _ = log_file.flush();
        println!("\n{}Log saved to {}{}{}", CYAN, BOLD, log_path, RESET);
    } else {
        println!("Please enter Port number");
        let Some(port_input_formatted) = read_port_or_menu("port number") else {
            return;
        };
        println!("Selected Port {}{}{}", CYAN, port_input_formatted, RESET);

        let (mut log_file, log_path) = create_log_file("single_port");
        write_log_header(&mut log_file, "Single Port Scan", ip_input);
        if let Some(note) = &resolution_note {
            write_log_entry(&mut log_file, note);
        }
        write_log_entry(
            &mut log_file,
            &format!("Target Port: {}", port_input_formatted),
        );

        println!(
            "Scanning Port {}{}{} on IP address {}{}{}",
            YELLOW, port_input_formatted, RESET, CYAN, ip_input, RESET
        );

        print!("Scanning... ");
        for _ in 0..3 {
            print!(".");
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(300));
        }

        let mut open_ports = Vec::new();

        match scan_port_ip(resolved_ip, port_input_formatted) {
            PortStatus::Open => {
                let service_name = get_service_name(port_input_formatted);
                println!(
                    " {}{}OPEN{} ({}{}{})",
                    GREEN, BOLD, RESET, CYAN, service_name, RESET
                );
                open_ports.push(port_input_formatted);
                write_log_entry(
                    &mut log_file,
                    &format!("Port {}: {} - OPEN", port_input_formatted, service_name),
                );
            }
            PortStatus::TimeoutFiltered => {
                println!(" {}TIMEOUT/FILTERED{}", YELLOW, RESET);
                write_log_entry(
                    &mut log_file,
                    &format!("Port {}: TIMEOUT/FILTERED", port_input_formatted),
                );
            }
            PortStatus::Closed => {
                println!(" {}CLOSED{}", RED, RESET);
                write_log_entry(
                    &mut log_file,
                    &format!("Port {}: CLOSED", port_input_formatted),
                );
            }
            PortStatus::InvalidAddress => {
                println!("\n{}Invalid address format{}", RED, RESET);
                write_log_entry(&mut log_file, "Error: Invalid address format");
            }
        }

        let elapsed = scan_started.elapsed().as_secs_f32();

        write_log_summary(&mut log_file, &open_ports, 1, elapsed);
        let _ = log_file.flush();
        println!("{}Scan took {:.2} seconds{}", CYAN, elapsed, RESET);
        println!("\n{}Log saved to {}{}{}", CYAN, BOLD, log_path, RESET);
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

const QUICK_PORTS: &[u16] = &[
    21, 22, 23, 25, 53, 80, 110, 143, 443, 445, 993, 995, 1723, 3306, 3389, 5900, 8080,
];

const WEB_PORTS: &[u16] = &[
    80, 443, 3000, 3001, 4200, 4443, 5000, 5001, 8000, 8008, 8080, 8081, 8088, 8443, 8888, 9000,
];

const DATABASE_PORTS: &[u16] = &[
    1433, 1521, 3306, 5432, 5984, 6379, 7000, 7001, 8086, 9042, 9200, 11211, 27017, 50000,
];

const FULL_PORTS: &[u16] = &[
    21, 22, 23, 25, 53, 67, 68, 80, 110, 111, 123, 135, 139, 143, 161, 389, 443, 445, 465, 514,
    587, 636, 993, 995, 1080, 1194, 1433, 1521, 1723, 1883, 3000, 3128, 3306, 3389, 5060, 5432,
    5672, 5900, 5984, 5985, 6379, 7000, 8080, 8086, 8443, 8888, 9092, 9200, 10000, 11211, 15672,
    27017,
];

impl ScanProfile {
    fn get_ports(&self) -> &'static [u16] {
        match self {
            ScanProfile::Quick => QUICK_PORTS,
            ScanProfile::Web => WEB_PORTS,
            ScanProfile::Database => DATABASE_PORTS,
            ScanProfile::Full => FULL_PORTS,
        }
    }

    fn get_name(&self) -> &'static str {
        match self {
            ScanProfile::Quick => "Quick Scan",
            ScanProfile::Web => "Web Services",
            ScanProfile::Database => "Database Services",
            ScanProfile::Full => "Full Common Ports",
        }
    }

    fn get_log_name(&self) -> &'static str {
        match self {
            ScanProfile::Quick => "profile_quick",
            ScanProfile::Web => "profile_web",
            ScanProfile::Database => "profile_database",
            ScanProfile::Full => "profile_full",
        }
    }
}

fn profile_scan() {
    clear_screen();
    let scan_started = Instant::now();

    println!(
        "{}{}=== Profile-Based Port Scanner ==={}\n",
        YELLOW, BOLD, RESET
    );

    println!("Please enter IP address or hostname:");
    let ip_input = match read_input("") {
        Ok(input) => input,
        Err(_) => {
            println!("{}Failed to read IP address{}", RED, RESET);
            menu_fallback();
            return;
        }
    };

    let (resolved_ip, resolution_note) = resolve_target_note(ip_input.as_str());

    println!("\n{}Select scan profile{}:", YELLOW, RESET);
    println!(
        "1. Quick Scan ({} ports)",
        ScanProfile::Quick.get_ports().len()
    );
    println!(
        "2. Web Services ({} ports)",
        ScanProfile::Web.get_ports().len()
    );
    println!(
        "3. Database Services ({} ports)",
        ScanProfile::Database.get_ports().len()
    );
    println!(
        "4. Full Scan ({} ports)",
        ScanProfile::Full.get_ports().len()
    );
    print!("\nYour choice: ");
    io::stdout().flush().unwrap();

    let profile_choice = match read_input("") {
        Ok(input) => input,
        Err(_) => {
            println!("{}Failed to read choice{}", RED, RESET);
            menu_fallback();
            return;
        }
    };

    let profile = match profile_choice.trim() {
        "1" => ScanProfile::Quick,
        "2" => ScanProfile::Web,
        "3" => ScanProfile::Database,
        "4" => ScanProfile::Full,
        _ => {
            println!("{}Invalid choice{}", RED, RESET);
            menu_fallback();
            return;
        }
    };

    let (mut log_file, log_path) = create_log_file(profile.get_log_name());
    write_log_header(&mut log_file, profile.get_name(), ip_input.as_str());
    if let Some(note) = &resolution_note {
        write_log_entry(&mut log_file, note);
    }

    let ports_to_scan = profile.get_ports();
    let total_ports = ports_to_scan.len();

    write_log_entry(&mut log_file, &format!("Profile: {}", profile.get_name()));
    write_log_entry(
        &mut log_file,
        &format!("Total ports to scan: {}", total_ports),
    );
    write_log_entry(&mut log_file, &format!("Ports: {:?}\n", ports_to_scan));

    println!(
        "\n{}{}{} - Scanning {}{}{} ports on {}{}{}",
        YELLOW,
        profile.get_name(),
        RESET,
        CYAN,
        total_ports,
        RESET,
        CYAN,
        ip_input,
        RESET
    );

    let mut open_ports = Vec::new();

    for (index, port) in ports_to_scan.iter().enumerate() {
        let percentage = ((index + 1) as f32 / total_ports as f32 * 100.0) as u32;
        let service_name = get_service_name(*port);

        print!("\rScanning {} ({})... ", service_name, port);
        io::stdout().flush().unwrap();

        match scan_port_ip(resolved_ip, *port) {
            PortStatus::Open => {
                print!("\r\x1b[2K");
                println!(
                    "{}✓{} {}{}{} ({}{}{}) - {}{}OPEN{}",
                    GREEN,
                    RESET,
                    CYAN,
                    service_name,
                    RESET,
                    YELLOW,
                    port,
                    RESET,
                    GREEN,
                    BOLD,
                    RESET
                );
                open_ports.push(*port);
                write_log_entry(
                    &mut log_file,
                    &format!("Port {}: {} - OPEN", port, service_name),
                );

                print!("Progress: [{}/{}] {}% ", index + 1, total_ports, percentage);
                print_progress_bar(percentage);
                io::stdout().flush().unwrap();
            }
            PortStatus::TimeoutFiltered => {
                write_log_entry(
                    &mut log_file,
                    &format!("Port {}: {} - TIMEOUT/FILTERED", port, service_name),
                );
                print!(
                    "\rProgress: [{}/{}] {}% ",
                    index + 1,
                    total_ports,
                    percentage
                );
                print_progress_bar(percentage);
                io::stdout().flush().unwrap();
            }
            PortStatus::Closed => {
                write_log_entry(
                    &mut log_file,
                    &format!("Port {}: {} - CLOSED", port, service_name),
                );
                print!(
                    "\rProgress: [{}/{}] {}% ",
                    index + 1,
                    total_ports,
                    percentage
                );
                print_progress_bar(percentage);
                io::stdout().flush().unwrap();
            }
            PortStatus::InvalidAddress => {
                write_log_entry(&mut log_file, &format!("Port {}: Invalid address", port));
                continue;
            }
        }
    }

    print!("\r\x1b[2K");

    let elapsed = scan_started.elapsed().as_secs_f32();

    println!(
        "\n{}{}{} Scan Complete!{}",
        GREEN,
        BOLD,
        profile.get_name(),
        RESET
    );
    println!("{}", "━".repeat(33));
    println!(
        "Found {}{}{} open ports out of {} scanned",
        GREEN,
        open_ports.len(),
        RESET,
        total_ports
    );
    println!("{}Scan took {:.2} seconds{}", CYAN, elapsed, RESET);

    if !open_ports.is_empty() {
        println!("\n{}Summary of open services{}:", YELLOW, RESET);
        for port in &open_ports {
            let service = get_service_name(*port);
            println!(
                "  {}•{} {}{:<15}{} on port {}{}{}",
                GREEN, RESET, CYAN, service, RESET, YELLOW, port, RESET
            );
        }
    } else {
        println!("\n{}No open ports found.{}", YELLOW, RESET);
    }

    write_log_summary(&mut log_file, &open_ports, total_ports as u32, elapsed);
    let _ = log_file.flush();
    println!("\n{}Log saved to {}{}{}", CYAN, BOLD, log_path, RESET);

    press_enter_to_continue();
}

fn print_progress_bar(percentage: u32) {
    let bar_width: usize = 20;
    let filled = bar_width * percentage as usize / 100;
    let empty = bar_width.saturating_sub(filled);

    print!("[");
    print!("{}{}{}", GREEN, "=".repeat(filled), RESET);
    if filled < bar_width {
        print!("{}>{}", YELLOW, RESET);
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
    println!(
        "{}{}{} v{}{}",
        CYAN, BOLD, my_scanner.name, my_scanner.version, RESET
    );
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
    println!("{}Please select option from the menu.{}", YELLOW, RESET);
    thread::sleep(Duration::from_millis(2000));
    print_menu_items();
}

fn end_program() {
    println!("\n{}Thank you for using Vonogs Scanner!{}", CYAN, RESET);
    println!("{}Goodbye!{}", GREEN, RESET);
    thread::sleep(Duration::from_millis(1000));
    process::exit(0);
}

fn press_enter_with_message(message: &str) {
    let mut stdout = stdout();
    let _ = write!(stdout, "\n{}{}{}", YELLOW, message, RESET);
    let _ = stdout.flush();
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);

    clear_screen();
    print_menu_items();
}

fn press_enter_to_continue() {
    press_enter_with_message("Press Enter to continue...");
}
