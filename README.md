# OpenConstruct Examples — Self-Contained Recipes for Every Language

Working examples showing how to use every part of the OpenConstruct ecosystem. Each example is standalone and runnable.

**Part of [SuperInstance OpenConstruct](https://github.com/SuperInstance/OpenConstruct).**

## What This Gives You

- **10 runnable examples** across Rust, Python, TypeScript, Go, C, and Arduino
- **Every major feature covered** — onboarding, fleet discovery, sense fusion, policy, workflows
- **Copy-paste ready** — each example compiles and runs independently
- **Real hardware** — ESP32 sensor node example with DHT22

## Quick Start

1. Install the SDK for your language (see below)
2. Have a coordinator running at `ws://localhost:9142` (override with `--coordinator`)
3. Run an example

## Examples

| # | File | Language | What It Demonstrates |
|---|------|----------|----------------------|
| 1 | [`rust/onboard.rs`](examples/rust/onboard.rs) | Rust | Basic onboarding, identity declaration, heartbeat loop |
| 2 | [`rust/fleet_scan.rs`](examples/rust/fleet_scan.rs) | Rust | Fleet discovery, topology printing, room/node listing |
| 3 | [`rust/sense_fusion.rs`](examples/rust/sense_fusion.rs) | Rust | Multi-sensor ingestion, shadow event fusion, occupancy grid |
| 4 | [`rust/policy_check.rs`](examples/rust/policy_check.rs) | Rust | Policy definition, upload, request evaluation |
| 5 | [`rust/workflow_dag.rs`](examples/rust/workflow_dag.rs) | Rust | DAG workflow definition, submission, execution monitoring |
| 6 | [`python/onboard.py`](examples/python/onboard.py) | Python | Thin-client onboarding, heartbeat loop with graceful shutdown |
| 7 | [`typescript/onboard.ts`](examples/typescript/onboard.ts) | TypeScript | Node.js onboarding with async heartbeat loop |
| 8 | [`go/onboard.go`](examples/go/onboard.go) | Go | Go SDK onboarding, context-based cancellation |
| 9 | [`c/onboard.c`](examples/c/onboard.c) | C | C ABI / FFI onboarding, manual memory management |
| 10 | [`esp32/sensor_node.ino`](examples/esp32/sensor_node.ino) | C++ | ESP32 WiFi sensor node, DHT22 readings |

## SDK Installation

```bash
# Rust
# Add to Cargo.toml: openconstruct = "0.1"

# Python
pip install openconstruct

# TypeScript
npm install @openconstruct/sdk

# Go
go get github.com/superinstance/openconstruct-go

# C
# Link against libopenconstruct (see c/ example)
```

## License

MIT
