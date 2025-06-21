pub fn is_valid_port(port: u16) -> bool {
    port > 0
}

pub fn is_valid_port_input(input: &str) -> bool {
    match input.trim().parse::<u16>() {
        Ok(port) => is_valid_port(port),
        Err(_) => false,
    }
}
