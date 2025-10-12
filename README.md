# Vonogs Sentinel

A network security toolkit written in Rust, focusing on network discovery and analysis.

## Features

- **Custom Port Scanning** — single port or port range
- **Profile Scans** — Quick/Web/Database/Full curated port sets
- **Common Service Names** — maps well-known ports to friendly labels (SSH, HTTP, MySQL, etc.)
- **IP or Hostname Targets** — type `192.168.1.10` or `example.com`
- **Coloured CLI & Progress Bar**
- **Timestamped Logs** — saved to `scan_logs/`
- **Configurable TCP Connect Timeout** — via `VONOGS_TIMEOUT_MS` (milliseconds)
- **IPv4 Support** (hostname resolution uses your OS resolver)

## Roadmap

- Multi-threading for faster scans
- Export formats (JSON/CSV)
- Additional scanning techniques
- Extended protocol support
- Performance optimisations
- Advanced service fingerprinting

## Installation

### From source

```bash
git clone https://github.com/alpibit/vonogs-sentinel
cd vonogs-sentinel
cargo build --release
```

The binary will be at:

```
./target/release/vonogs
```

You can also run via Cargo:

```bash
cargo run --release
```

Or install to your Cargo bin directory:

```bash
cargo install --path .
~/.cargo/bin/vonogs
```

(Optionally add `~/.cargo/bin` to your `PATH`.)

## Usage

Run the programme and follow the interactive menu:

```bash
./target/release/vonogs
```

### Scan options

1. **Custom Port Scan** — scan a single port or a port range on a target  
2. **Profile Scan** — pick a predefined port set (Quick, Web, Database, Full)  
3. **Exit**

Targets may be **IPv4 addresses or hostnames**.  
Valid ports are **1..=65535**.

### Examples

- **Single port on a hostname**
  - Target: `example.com`
  - Port: `443`

- **Range scan**
  - Target: `192.168.1.50`
  - Range: `1–1024`

- **Web profile**
  - Target: `my.internal.host`
  - Select: `Web Services`

## Timeout configuration

Control the TCP connect timeout (per port) using an environment variable:

```bash
# Default (if not set) is 700ms
VONOGS_TIMEOUT_MS=1200 ./target/release/vonogs
```

Use a **lower** value for snappier scans on responsive networks, or a **higher** value for high-latency or packet-dropping networks.

## Logs

Each run writes a timestamped log file into `scan_logs/`, including:

- Target, start/end time  
- Per-port results  
- Summary with open ports and service names

## Validation rules

- Ports must be **1..=65535** (port `0` is rejected).
- Non-numeric or out-of-range input is rejected.
- For ranges, **start ≤ end** is required.
- If the target isn’t a literal IP, a DNS lookup is attempted. If resolution fails, the log will contain **“Invalid address”** entries.

## Troubleshooting

- **Binary not executing**: ensure you’re running `./target/release/vonogs` from the project folder.
- **Colours look odd**: your terminal must support ANSI escape codes (most do by default).
- **“Invalid address”**: check DNS resolution (`getent hosts <name>` on Linux) or try a literal IP.
- **Slow scans**: increase `VONOGS_TIMEOUT_MS` or use tighter port ranges/profiles.

## Licence

This project is licensed under the MIT Licence.