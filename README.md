# OpenConstruct Examples

Working examples showing how to use every part of the OpenConstruct ecosystem. Each example is self-contained and runnable.

## Getting Started

1. **Have a running coordinator** — most examples connect to `ws://localhost:9142`. Override with `--coordinator`.
2. **Install the SDK for your language:**
   - **Rust:** Add `openconstruct` to your `Cargo.toml` dependencies
   - **Python:** `pip install openconstruct`
   - **TypeScript:** `npm install @openconstruct/sdk`
   - **Go:** `go get github.com/openconstruct/go-sdk`
   - **C:** Link against `libopenconstruct` (see the example for build instructions)
   - **ESP32:** Arduino IDE with ESP32 board support + required libraries
3. **Run an example** — see the language-specific sections below.

## Examples

| # | File | Language | What It Demonstrates |
|---|------|----------|----------------------|
| 1 | [`examples/rust/onboard.rs`](examples/rust/onboard.rs) | Rust | Basic agent onboarding, identity declaration, heartbeat loop |
| 2 | [`examples/rust/fleet_scan.rs`](examples/rust/fleet_scan.rs) | Rust | Fleet discovery, topology printing, room/node listing |
| 3 | [`examples/rust/sense_fusion.rs`](examples/rust/sense_fusion.rs) | Rust | Multi-sensor ingestion (vision + sonar), shadow event fusion, occupancy grid |
| 4 | [`examples/rust/policy_check.rs`](examples/rust/policy_check.rs) | Rust | Policy definition, upload, and request evaluation (allow/deny) |
| 5 | [`examples/rust/workflow_dag.rs`](examples/rust/workflow_dag.rs) | Rust | DAG workflow definition, submission, execution monitoring |
| 6 | [`examples/python/onboard.py`](examples/python/onboard.py) | Python | Thin-client onboarding, heartbeat loop with graceful shutdown |
| 7 | [`examples/typescript/onboard.ts`](examples/typescript/onboard.ts) | TypeScript | Node.js onboarding with async heartbeat loop |
| 8 | [`examples/go/onboard.go`](examples/go/onboard.go) | Go | Go SDK onboarding, context-based cancellation, signal handling |
| 9 | [`examples/c/onboard.c`](examples/c/onboard.c) | C | C ABI / FFI onboarding, manual memory management, error handling |
| 10 | [`examples/esp32/sensor_node.ino`](examples/esp32/sensor_node.ino) | C++ (Arduino) | ESP32 WiFi sensor node, DHT22 readings, WebSocket coordinator connection |

## By Topic

### Onboarding
The first thing every agent does — register with the coordinator and start heartbeating. Available in [Rust](examples/rust/onboard.rs), [Python](examples/python/onboard.py), [TypeScript](examples/typescript/onboard.ts), [Go](examples/go/onboard.go), and [C](examples/c/onboard.c).

### Fleet Management
- **[`fleet_scan.rs`](examples/rust/fleet_scan.rs)** — Discover all rooms and nodes in the fleet, print a topology tree.

### Sensing & Fusion
- **[`sense_fusion.rs`](examples/rust/sense_fusion.rs)** — Ingest vision and sonar shadow events, fuse into an occupancy grid.
- **[`sensor_node.ino`](examples/esp32/sensor_node.ino)** — Real hardware: ESP32 publishing DHT22 readings to the coordinator.

### Policy & Access Control
- **[`policy_check.rs`](examples/rust/policy_check.rs)** — Define allow/deny policies, evaluate requests, see audit decisions.

### Workflows
- **[`workflow_dag.rs`](examples/rust/workflow_dag.rs)** — Build a multi-task DAG, submit it, and watch execution progress.

## License

MIT
