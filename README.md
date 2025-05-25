# Vonogs Sentinel

A network security toolkit written in Rust, focusing on network discovery and analysis.

## Features

- **Custom Port Scanning** - Single port or port range scanning
- **Quick Scan** - Fast scan of top 20 most common ports
- **Common Services Detection** - Identifies services by name (SSH, HTTP, MySQL, etc.)
- **Interactive Menu** - Easy-to-use command-line interface
- **IPv4 Support**

## Roadmap

- Multi-threading for faster scans 
- Export scan results to file
- Additional scanning techniques
- Extended protocol support
- Performance optimisations
- Advanced service fingerprinting

## Installation

### From Source

```bash
git clone https://github.com/alpibit/vonogs-sentinel
cd vonogs-sentinel
cargo build --release
```

## Usage

Run the program and follow the interactive menu:

```bash
./target/release/vonogs-sentinel
```

### Scan Options

1. **Custom Port Scan** - Scan single port or port range on target IP
2. **Quick Scan** - Scan top 20 ports (fast reconnaissance)
3. **Common Services** - Identify running services on common ports

## Licence

This project is licenced under the MIT Licence.