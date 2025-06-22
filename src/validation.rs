use std::net::IpAddr;

pub fn is_valid_port(port: u16) -> bool {
    port > 0
}

pub fn is_valid_port_input(input: &str) -> bool {
    match input.trim().parse::<u16>() {
        Ok(port) => is_valid_port(port),
        Err(_) => false,
    }
}

pub fn is_valid_ip(ip: &str) -> bool {
    ip.trim().parse::<IpAddr>().is_ok()
}
