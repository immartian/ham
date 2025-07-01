# HAM - Heuristic Adaptive Monitor

**A censorship-aware, terminal-native tool that empowers users under internet restrictions to scan, analyze, and tunnel through blocked networks with minimal effort.**

[![Rust](https://img.shields.io/badge/rust-latest-brightgreen.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Overview

HAM acts like a "digital survival radio" for the internet, combining protocol scanning, local diagnostics, and tunnel orchestration into one adaptive CLI utility. Inspired by the functionality and spirit of HAM radio, it provides real-time feedback on network conditions and helps users navigate internet censorship.

## Features

### MVP Features (Available Now)

- **Live Protocol Scanning**: Real-time monitoring of TCP, UDP, DNS, and HTTPS connectivity
- **Network Analysis**: Comprehensive diagnostics to distinguish between local issues and censorship
- **Censorship Detection**: Heuristic analysis to identify filtering patterns
- **Configuration Export**: Share working configurations via JSON
- **Terminal UI**: Color-coded, updating interface with progress bars

### Planned Features

- **Tunnel Orchestration**: Automatic selection and setup of best escape methods
- **QR Code Export**: Easy sharing of bridge configurations
- **Live Dashboard**: Real-time TUI with continuous updates  
- **Community Bridges**: Decentralized sharing of working escape routes

## Quick Start

### Prerequisites

- Rust 1.70+ 
- Linux/macOS/Windows (tested on Linux)

### Installation

```bash
git clone https://github.com/your-org/ham.git
cd ham
cargo build --release
```

### Usage

```bash
# Live protocol monitoring (press 'q' to quit)
./target/release/ham scan

# Network diagnostics
./target/release/ham analyze  

# Export configuration
./target/release/ham export json
```

## Commands

### `ham scan`
Interactive real-time monitoring of network protocols:
```
HAM - Network Protocol Scanner
Press 'q' to quit

[TCP:80  ] ██████████ Good
[TCP:443 ] ███████░░░ Good (SNI-sensitive)  
[DNS     ] ██████████ Good
[PING    ] ██████████ Good
[UDP     ] ████████░░ Good
```

### `ham analyze`
Comprehensive network analysis:
```
HAM Network Analysis
Analyzing network conditions...

Network Interface Status:
   ✓ Default route found

Connectivity Tests:
   ✓ Google DNS - Reachable
   ✓ Cloudflare DNS - Reachable
   ✓ OpenDNS - Reachable

Censorship Detection:
   Network appears uncensored
```

### `ham export`
Export working configurations:
```bash
# JSON format
ham export json

# QR code (planned)
ham export qr --output bridge.qr
```

## Technical Details

### Detection Methods

- **DNS Resolution**: Tests for DNS poisoning and hijacking
- **TCP Connectivity**: Port reachability testing to common services
- **HTTPS Analysis**: TLS handshake and certificate validation
- **Network Interface**: Local routing and interface status
- **Censorship Heuristics**: Pattern recognition for common blocking techniques

### Architecture

- **Language**: Rust for safety, speed, and portability
- **Async Runtime**: Tokio for concurrent network operations
- **Terminal UI**: Crossterm for cross-platform terminal control
- **HTTP Client**: Reqwest for HTTPS connectivity testing
- **CLI Framework**: Clap for command-line interface

## Demo

Run the included demo to see all features:

```bash
./demo.sh
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build  
cargo build --release

# Run tests
cargo test

# Check code
cargo check
```

### Dependencies

Key dependencies include:
- `tokio` - Async runtime
- `reqwest` - HTTP client
- `crossterm` - Terminal interface
- `clap` - CLI framework
- `colored` - Terminal colors
- `dns-lookup` - DNS resolution

## Contributing

We welcome contributions from:

- **Users under censorship** who can provide test data
- **Rust developers** interested in freedom tech
- **Networking experts** to advise probe design  
- **UX contributors** for terminal experience
- **Privacy advocates** to audit tactics

### Development Roadmap

- [ ] Real-time TUI dashboard with continuous updates
- [ ] Tunnel module integration (proxychains, Tor, obfs4)
- [ ] QR code generation for bridge sharing
- [ ] Community bridge exchange protocol
- [ ] Protocol fingerprint analysis
- [ ] Bandwidth throttling detection

## Background

HAM addresses the growing need for censorship circumvention tools in environments where internet freedom is restricted. Traditional tools often require technical expertise or leave users guessing about network conditions. HAM bridges this gap by providing:

1. **Clear visibility** into what's blocked and what works
2. **Automated diagnostics** to distinguish censorship from technical issues  
3. **Adaptive strategies** that evolve with blocking techniques
4. **Minimal overhead** suitable for resource-constrained environments

## Security Considerations

- Network probes use minimal, non-invasive techniques
- No persistent data collection or tracking
- Local-only operation by default
- Optional encrypted bridge sharing (planned)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by HAM radio operators and their resilient communication networks
- Built on the shoulders of open-source circumvention tools
- Dedicated to internet freedom advocates worldwide

---

**"Let's build the diagnostic survival radio for the censored internet age."**

For more information, see [prd.md](prd.md) for the complete project requirements and vision.
