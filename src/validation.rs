use std::net::IpAddr;

pub fn is_valid_port(port: u16) -> bool {
    port != 0
}

pub fn is_valid_ip(ip: &str) -> bool {
    ip.trim().parse::<IpAddr>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_zero_is_invalid() {
        assert!(!is_valid_port(0));
    }

    #[test]
    fn normal_ports_are_valid() {
        assert!(is_valid_port(1));
        assert!(is_valid_port(80));
        assert!(is_valid_port(65535));
    }

    #[test]
    fn accepts_ipv4_and_ipv6() {
        assert!(is_valid_ip("127.0.0.1"));
        assert!(is_valid_ip("192.168.1.50"));
        assert!(is_valid_ip("::1"));
    }

    #[test]
    fn surrounding_whitespace_is_trimmed() {
        assert!(is_valid_ip("  8.8.8.8  "));
        assert!(is_valid_ip("\t8.8.4.4\n"));
    }

    #[test]
    fn rejects_hostnames() {
        assert!(!is_valid_ip("example.com"));
        assert!(!is_valid_ip("localhost"));
    }

    #[test]
    fn rejects_malformed_addresses() {
        assert!(!is_valid_ip("999.1.1.1"));
        assert!(!is_valid_ip("1.2.3"));
        assert!(!is_valid_ip(""));
    }
}
