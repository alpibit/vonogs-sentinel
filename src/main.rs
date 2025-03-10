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
                "2" => end_program(),
                _ => menu_fallback(),
            },
            Err(_) => menu_fallback(),
        }
    }
}

fn scanner() {
    clear_screen();
    let mut port_input = String::new();
    let mut ip_input = String::new();

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

    let port_input_formatted = match port_input.trim().parse::<u16>() {
        Ok(port) => port,
        Err(_) => {
            println!("Failed to read port");
            menu_fallback();
            return;
        }
    };

    let ip_input = ip_input.trim();

    println!(
        "Scanning Port {} on IP address {}",
        port_input_formatted, ip_input
    );

    let socket_addr = match format!("{}:{}", ip_input, port_input_formatted).parse::<SocketAddr>() {
        Ok(addr) => addr,
        Err(_) => {
            println!("Invalid address format");
            return;
        }
    };

    match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(3)) {
        Ok(_) => println!("Port {} is OPEN", port_input_formatted),
        Err(_) => println!("Port {} is CLOSED", port_input_formatted),
    }

    press_enter();
}

enum MenuItem {
    SinglePortScan,
    Exit,
}

impl MenuItem {
    fn get_description(&self) -> String {
        match self {
            MenuItem::SinglePortScan => String::from("1. Single Port Scan"),
            MenuItem::Exit => String::from("2. Exit"),
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
        version: 0.1,
    };
    println!("{} Scanner: v{}", my_scanner.name, my_scanner.version);
    println!("------------------");

    let menu: [MenuItem; 2] = [MenuItem::SinglePortScan, MenuItem::Exit];

    for item in &menu {
        println!("{}", item.get_description());
    }
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
    println!("Vonogs is closing!");
    process::exit(0x0100);
}

fn press_enter() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();

    clear_screen();

    print_menu_items();
}
