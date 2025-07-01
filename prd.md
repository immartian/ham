# Project: HAM (Heuristic Adaptive Monitor)

## Summary

**HAM** is a modular, configurable, and pluggable censorship-aware terminal tool that empowers users under internet restrictions to scan, analyze, and tunnel through blocked networks with methodical precision. Inspired by the functionality and spirit of HAM radio, HAM acts like a "digital survival radio" for the internet, combining protocol scanning, heuristic analysis, and adaptive reasoning into one extensible CLI utility.

---

## Problem Statement

In heavily censored environments (e.g. Iran, China, Russia), users face a variety of evolving restrictions:

* Protocol-level filtering (e.g., QUIC, TLS fingerprinting)
* Port blocking (e.g., 443, 80, 8080)
* IP/ASN filtering
* Upload throttling or asymmetric blocking
* DNS poisoning or hijacking
* IPv6 or ICMP disabling

Yet users are often left guessing: "Is it my Wi-Fi? DNS? Censorship? The app?"

HAM closes this awareness gap through systematic analysis, configurable testing, and automated reasoning about network conditions.

---

## Vision

HAM provides:

* **Modular architecture** for extensible protocol testing and analysis
* **Configurable behavior** through comprehensive configuration management
* **Pluggable systems** for adding new protocols, tests, and reasoning engines
* **Heuristic analysis** that learns and adapts to censorship patterns
* **Methodical reasoning** to distinguish between different types of network issues
* **Real-time feedback** on protocol availability with intelligent scoring
* **Tactical remediation** using existing open-source tools (e.g. proxychains, obfs4proxy, sultry)

---

## Architecture Principles

### 0. Modularization

HAM follows a clean modular architecture:

```
ham/
├── core/           # Core engine and interfaces
├── protocols/      # Protocol-specific modules
├── analyzers/      # Heuristic analysis engines
├── config/         # Configuration management
├── ui/            # Terminal UI components
├── tunnels/       # Tunnel orchestration
└── plugins/       # Plugin system for extensions
```

**Modules:**
- **Protocol Module**: TCP, UDP, DNS, HTTPS, QUIC, etc.
- **Analysis Module**: Censorship detection, bandwidth analysis, latency profiling
- **Reasoning Module**: Machine learning, pattern recognition, decision trees
- **Configuration Module**: YAML/JSON config, environment variables, CLI overrides
- **UI Module**: Terminal rendering, progress tracking, interactive modes

### 1. Configurable

HAM is fully configurable at multiple levels:

**Configuration Hierarchy:**
1. **Global Config** (`~/.ham/config.yaml`)
2. **Project Config** (`./ham.yaml`) 
3. **Environment Variables** (`HAM_*`)
4. **CLI Arguments** (highest priority)

**Sample Configuration:**
```yaml
# ~/.ham/config.yaml
ham:
  scanning:
    interval: 2s
    timeout: 5s
    retries: 3
    parallel_tests: 10
    
  protocols:
    tcp:
      enabled: true
      ports: [80, 443, 8080, 22]
      endpoints: ["8.8.8.8", "1.1.1.1"]
    
    dns:
      enabled: true
      resolvers: ["8.8.8.8", "1.1.1.1", "9.9.9.9"]
      test_domains: ["google.com", "cloudflare.com"]
      
    https:
      enabled: true
      test_urls: ["https://google.com", "https://github.com"]
      verify_certificates: true
      
  analysis:
    censorship_threshold: 0.3
    confidence_levels: [0.7, 0.9, 0.95]
    learning_enabled: true
    
  ui:
    color_mode: "auto"  # auto, always, never
    refresh_rate: 500ms
    progress_style: "bars"  # bars, dots, percentage
    
  export:
    formats: ["json", "yaml", "qr"]
    encryption: true
    compression: true
```

### 2. Pluggable

HAM supports extensible plugin architecture:

**Protocol Plugins:**
```rust
// plugins/protocols/wireguard.rs
pub struct WireGuardProtocol;

impl ProtocolTest for WireGuardProtocol {
    fn name(&self) -> &str { "WireGuard" }
    
    async fn test(&self, config: &Config) -> TestResult {
        // Custom WireGuard connectivity test
    }
    
    fn score(&self, result: &TestResult) -> u8 {
        // Custom scoring logic
    }
}
```

**Analysis Plugins:**
```rust
// plugins/analyzers/deep_packet_inspection.rs
pub struct DPIDetector;

impl HeuristicAnalyzer for DPIDetector {
    async fn analyze(&self, data: &NetworkData) -> AnalysisResult {
        // DPI detection logic
    }
}
```

**Plugin Registration:**
```yaml
# plugins.yaml
plugins:
  protocols:
    - name: "wireguard"
      path: "./plugins/protocols/wireguard.so"
      config: { port: 51820 }
      
  analyzers:
    - name: "dpi_detector"  
      path: "./plugins/analyzers/dpi.so"
      priority: high
```

### 3. Heuristic Analysis & Reasoning

HAM employs sophisticated analysis and reasoning:

**Multi-Layer Analysis:**
1. **Protocol Layer**: Individual protocol connectivity and performance
2. **Pattern Layer**: Cross-protocol correlation and censorship signatures  
3. **Temporal Layer**: Time-based analysis and trend detection
4. **Behavioral Layer**: User behavior impact and adaptation strategies

**Reasoning Engine:**
```rust
pub struct ReasoningEngine {
    decision_tree: DecisionTree,
    pattern_matcher: PatternMatcher,
    ml_model: Option<MLModel>,
    confidence_calculator: ConfidenceCalculator,
}

impl ReasoningEngine {
    pub async fn analyze_network_state(&self, data: &NetworkData) -> NetworkDiagnosis {
        let protocol_scores = self.analyze_protocols(data).await;
        let censorship_indicators = self.detect_censorship_patterns(data).await;
        let confidence = self.calculate_confidence(&protocol_scores, &censorship_indicators);
        
        NetworkDiagnosis {
            overall_status: self.determine_overall_status(&protocol_scores),
            censorship_likelihood: self.estimate_censorship(censorship_indicators),
            recommended_actions: self.suggest_actions(&protocol_scores),
            confidence_level: confidence,
            reasoning: self.explain_reasoning(&protocol_scores, &censorship_indicators),
        }
    }
}
```

**Heuristic Rules:**
- **DNS Poisoning Detection**: Compare responses from multiple resolvers
- **Bandwidth Throttling**: Measure upload/download asymmetry 
- **DPI Fingerprinting**: Analyze connection reset patterns
- **Geographic Correlation**: Compare results across regions
- **Temporal Analysis**: Track changes over time to detect new blocks

---

## Core Features

HAM's modular architecture enables methodical network analysis through specialized components:

### 1. Scan Module - Real-time Protocol Monitoring

```bash
ham scan --config ~/.ham/scan.yaml
```

**Configurable Live Monitoring:**
- **Protocol Discovery**: Dynamically loads enabled protocol plugins
- **Intelligent Scoring**: Uses weighted heuristics to score connectivity (0-10)
- **Adaptive Refresh**: Configurable scan intervals based on network stability
- **Parallel Testing**: Concurrent protocol tests for faster results

```
HAM - Network Protocol Scanner
Press 'q' to quit

[TCP:80  ] ██████████ Good       (10/10) - HTTP connectivity excellent
[TCP:443 ] ███████░░░ Limited    (7/10)  - HTTPS works, SNI filtering detected
[DNS     ] ██████████ Good       (10/10) - All resolvers responding
[QUIC    ] ██░░░░░░░░ Blocked    (2/10)  - Port blocking + DPI detected
[UDP     ] ████████░░ Good       (8/10)  - Some packet loss observed
[SSH     ] ░░░░░░░░░░ Failed     (0/10)  - Port 22 completely blocked

Confidence: 95% | Censorship Detected: High | Auto-refresh: 2s
```

**Plugin-based Protocol Support:**
- **Core Protocols**: TCP, UDP, DNS, HTTPS, ICMP
- **Pluggable Extensions**: SSH, QUIC, WireGuard, Tor, I2P
- **Custom Tests**: User-defined protocol testing modules

### 2. Analyze Module - Heuristic Network Diagnosis

```bash
ham analyze --depth=deep --config=./analysis.yaml
```

**Multi-dimensional Analysis:**

**Network Layer Analysis:**
```
📊 Network Interface Status:
   ✓ Default route found (via 192.168.1.1)
   ✓ IPv4 connectivity: Excellent
   ⚠ IPv6 connectivity: Disabled (policy-based)
   ✓ MTU discovery: 1500 bytes optimal
```

**Protocol Correlation Analysis:**
```
🔍 Cross-Protocol Correlation:
   📈 TCP success rate: 85% (good)
   📉 UDP success rate: 45% (suspicious - possible DPI)
   🎯 DNS consistency: 98% (reliable)
   ⚠ TLS handshake patterns: SNI filtering detected
```

**Censorship Pattern Detection:**
```
🛡️ Censorship Analysis:
   🚨 DPI Signatures Detected:
      - TLS ClientHello resets on specific domains
      - UDP packet drops above 1KB payload
      - DNS response TTL manipulation detected
   
   📊 Confidence Matrix:
      DNS Poisoning:      15% (low)
      Bandwidth Limiting: 75% (high) 
      Protocol Blocking:  90% (very high)
      Geographic Blocking: 25% (low)

   💡 Recommended Actions:
      1. Use TLS fragmentation (success probability: 85%)
      2. Switch to DoH/DoT for DNS (success probability: 70%)
      3. Consider QUIC alternatives (success probability: 20%)
```

**Reasoning Chain:**
```
🧠 Analysis Reasoning:
   1. TCP:443 works but shows connection resets → SNI filtering
   2. UDP severely degraded vs TCP → DPI targeting UDP
   3. DNS resolvers show consistent responses → no DNS poisoning
   4. Combined pattern matches: Great Firewall-style filtering
   
   Conclusion: Sophisticated DPI-based censorship (confidence: 95%)
```

### 3. Configuration Module - Adaptive Behavior Control

**Hierarchical Configuration:**
```yaml
# ~/.ham/config.yaml - Global defaults
scanning:
  default_timeout: 5s
  retry_count: 3
  parallel_limit: 10

protocols:
  tcp:
    enabled: true
    ports: [80, 443, 8080, 22, 993]
    test_endpoints: 
      - "8.8.8.8"
      - "1.1.1.1" 
      - "cloudflare.com"
    custom_headers:
      user_agent: "Mozilla/5.0 (compatible; HAM/1.0)"

  dns:
    enabled: true
    resolvers: ["8.8.8.8", "1.1.1.1", "9.9.9.9"]
    test_queries: ["google.com", "facebook.com", "youtube.com"]
    query_types: ["A", "AAAA", "TXT"]

heuristics:
  censorship_threshold: 0.7
  confidence_calculation: "weighted_average"
  pattern_learning: true
  temporal_analysis: true

analysis:
  reasoning_engine: "decision_tree"
  correlation_window: "5m"
  confidence_levels: [0.6, 0.8, 0.95]
```

**Runtime Configuration Override:**
```bash
# Environment variables
export HAM_SCAN_TIMEOUT=10s
export HAM_PROTOCOLS_TCP_ENABLED=false

# CLI overrides (highest priority)
ham scan --timeout=3s --protocols=dns,https --parallel=5
```

### 4. Plugin Module - Extensible Protocol & Analysis Framework

**Protocol Plugin Interface:**
```rust
// plugins/protocols/custom_protocol.rs
#[derive(Debug)]
pub struct CustomProtocol {
    config: ProtocolConfig,
}

#[async_trait]
impl ProtocolTest for CustomProtocol {
    async fn test(&self, target: &TestTarget) -> TestResult {
        // Custom connectivity testing logic
        let start = Instant::now();
        let result = self.execute_custom_test(target).await;
        
        TestResult {
            success: result.is_ok(),
            latency: start.elapsed(),
            error: result.err(),
            metadata: self.collect_metadata(&result),
        }
    }
    
    fn score(&self, result: &TestResult) -> ProtocolScore {
        // Custom scoring algorithm
        ProtocolScore {
            value: self.calculate_score(result),
            confidence: self.calculate_confidence(result),
            reasoning: self.explain_score(result),
        }
    }
}

// Plugin registration
ham_plugin_register!(CustomProtocol, "custom_protocol", "1.0.0");
```

**Analysis Plugin Interface:**
```rust
// plugins/analyzers/ml_classifier.rs
pub struct MLClassifier {
    model: TensorFlowModel,
    feature_extractor: FeatureExtractor,
}

#[async_trait] 
impl HeuristicAnalyzer for MLClassifier {
    async fn analyze(&self, data: &NetworkData) -> AnalysisResult {
        let features = self.feature_extractor.extract(data);
        let prediction = self.model.predict(&features).await?;
        
        AnalysisResult {
            classification: prediction.class,
            confidence: prediction.confidence,
            features: features,
            explanation: self.generate_explanation(&prediction),
        }
    }
}
```

### 5. Tunnel Module - Intelligent Circumvention (Planned)

```bash
ham tunnel --auto --target="ssh user@server.com"
```

**Adaptive Tunnel Selection:**
- **Success Probability Estimation**: ML-based tunnel effectiveness prediction
- **Automatic Fallback**: Try multiple methods in priority order  
- **Performance Optimization**: Choose fastest working method
- **Stealth Optimization**: Prefer least detectable methods

**Supported Tunnels:**
- **SOCKS/HTTP Proxies**: via proxychains integration
- **TLS Tunneling**: via stunnel/sultry 
- **Tor Integration**: obfs4, meek, snowflake bridges
- **Custom Protocols**: WireGuard, V2Ray, Shadowsocks

### 6. Export Module - Configuration & Intelligence Sharing

```bash
ham export --format=qr --encrypt --target=bridges.qr
```

**Multi-format Export:**
- **JSON/YAML**: Machine-readable configurations
- **QR Codes**: Mobile-friendly bridge sharing
- **Encrypted Bundles**: Secure configuration distribution
- **Mesh Sharing**: P2P configuration exchange (planned)

**Export Content:**
- Working tunnel configurations
- Censorship pattern signatures
- Network topology insights
- Success probability matrices

---

## Detection Tactics & Methodologies

HAM employs a methodical, multi-layered approach to network analysis:

### 1. Protocol-Specific Detection Methods

**TCP Analysis:**
- **Connection Establishment**: Three-way handshake timing analysis
- **Port Accessibility**: Systematic scanning of common service ports
- **RST Pattern Analysis**: Detecting DPI-triggered connection resets
- **MTU Discovery**: Path MTU detection for fragmentation analysis

**DNS Analysis:**  
- **Multi-Resolver Comparison**: Cross-validation using multiple DNS servers
- **Response Consistency**: Detecting poisoned or manipulated responses
- **Query Type Testing**: A, AAAA, CNAME, TXT record accessibility
- **TTL Manipulation Detection**: Identifying artificial TTL modifications
- **DoH/DoT Testing**: DNS-over-HTTPS and DNS-over-TLS connectivity

**HTTPS/TLS Analysis:**
- **Certificate Validation**: MITM detection through certificate chain analysis
- **SNI Filtering Detection**: Testing with various Server Name Indicators
- **ClientHello Fingerprinting**: Analyzing connection reset patterns
- **TLS Version Negotiation**: Testing different protocol versions
- **Cipher Suite Testing**: Identifying blocked cryptographic algorithms

**UDP Analysis:**
- **Packet Size Sensitivity**: Testing various payload sizes for DPI thresholds
- **Protocol Discrimination**: Distinguishing between UDP protocols (DNS, QUIC, VPN)
- **Rate Limiting Detection**: Measuring throughput degradation patterns
- **Stateful Inspection**: Testing for connection tracking behaviors

### 2. System Integration & Tools

**Native System Integration:**
- **Network Interface Monitoring**: Real-time interface statistics via `/proc/net/dev`
- **Routing Table Analysis**: Dynamic routing changes via `ip route` and `netlink`
- **Process Network Monitoring**: Connection tracking via `netstat`/`ss`
- **Firewall Integration**: `iptables`/`nftables` rule analysis

**External Tool Integration:**
- **DNS Tools**: `dig`, `nslookup`, `host` for comprehensive DNS testing
- **Network Tools**: `ping`, `traceroute`, `mtr` for path analysis  
- **TLS Tools**: `openssl`, `gnutls-cli` for certificate and handshake testing
- **Traffic Analysis**: `tcpdump`, `wireshark` integration for packet capture

---

## Technology Stack & Architecture

### Core Technologies

**Language & Runtime:**
- **Rust 1.70+**: Memory safety, performance, and async support
- **Tokio**: Asynchronous runtime for concurrent network operations

**Network Libraries:**
- **Reqwest**: HTTP/HTTPS client with connection pooling
- **Trust-DNS**: Pure Rust DNS client and server implementation
- **Rustls**: Modern TLS library for secure connections

**Terminal Interface:**
- **Crossterm**: Cross-platform terminal manipulation
- **Clap**: Command-line argument parsing with derive macros
- **Colored**: Terminal color output

**Data & Configuration:**
- **Serde**: Serialization/deserialization framework
- **DNS-lookup**: System DNS resolution

### Modular Architecture

```
ham/
├── core/           # Core engine and interfaces
├── protocols/      # Protocol-specific modules (TCP, DNS, HTTPS, UDP)
├── analyzers/      # Heuristic analysis engines
├── config/         # Configuration management
├── ui/            # Terminal UI components
├── tunnels/       # Tunnel orchestration (planned)
└── plugins/       # Plugin system for extensions (planned)
```

---

## Development Roadmap

### Phase 1: Foundation (✅ Completed)
- [x] **Core Architecture**: Modular design with async support
- [x] **Basic Protocols**: TCP, DNS, HTTPS, UDP testing
- [x] **Terminal UI**: Real-time scanning interface with progress bars
- [x] **Configuration System**: Structured configuration management
- [x] **Heuristic Analysis**: Basic censorship detection and reasoning

### Phase 2: Intelligence (📋 Next)  
- [ ] **Advanced Heuristics**: Pattern recognition and correlation analysis
- [ ] **Plugin System**: Dynamic protocol and analyzer loading
- [ ] **Temporal Analysis**: Time-series censorship tracking
- [ ] **Confidence Scoring**: Probabilistic result assessment
- [ ] **Configuration Files**: YAML-based external configuration

### Phase 3: Circumvention (🔮 Future)
- [ ] **Tunnel Integration**: Automatic circumvention tool orchestration
- [ ] **Success Prediction**: Effectiveness estimation for different methods
- [ ] **QR Code Export**: Mobile-friendly bridge sharing
- [ ] **Community Bridge Exchange**: Decentralized configuration sharing

---

## Call for Contributors

We’re looking for:

* Users under censorship who can provide test data
* Rust developers interested in freedom tech
* Networking experts to advise probe design
* UX contributors for terminal experience
* Privacy-aware collaborators to audit tactics

**Let’s build the diagnostic survival radio for the censored internet age.**
