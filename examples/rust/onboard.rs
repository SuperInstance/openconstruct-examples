//! # OpenConstruct — Basic Agent Onboarding (Rust)
//!
//! Demonstrates how an agent registers itself with the OpenConstruct
//! coordinator, receives its assigned room topology, and enters the
//! heartbeat loop.
//!
//! ## Usage
//! ```sh
//! cargo run --example onboard -- --coordinator ws://localhost:9142
//! ```

use anyhow::{Context, Result};
use clap::Parser;
use openconstruct::{
    AgentConfig, AgentIdentity, Capability, CoordinatorClient, HeartbeatConfig,
};
use std::time::Duration;
use tracing::{info, warn};

/// Command-line arguments for the onboarding example.
#[derive(Parser)]
#[command(name = "onboard", about = "Basic agent onboarding example")]
struct Args {
    /// WebSocket URL of the OpenConstruct coordinator.
    #[arg(long, default_value = "ws://localhost:9142")]
    coordinator: String,

    /// Human-readable name for this agent.
    #[arg(long, default_value = "example-agent")]
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // ── Initialise structured logging ───────────────────────────────
    tracing_subscriber::fmt()
        .with_env_filter("openconstruct=debug,info")
        .init();

    let args = Args::parse();

    // ── Build the coordinator client ────────────────────────────────
    let client = CoordinatorClient::connect(&args.coordinator)
        .await
        .context("failed to connect to coordinator")?;
    info!(url = %args.coordinator, "connected to coordinator");

    // ── Declare our identity and capabilities ───────────────────────
    let identity = AgentIdentity {
        name: args.name.clone(),
        capabilities: vec![Capability::Sense, Capability::Act],
        metadata: serde_json::json!({
            "language": "rust",
            "example": "onboard",
        }),
    };

    // ── Onboard ─────────────────────────────────────────────────────
    let config = AgentConfig {
        heartbeat: HeartbeatConfig {
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(10),
        },
        ..Default::default()
    };

    let session = client
        .onboard(identity, config)
        .await
        .context("onboarding handshake failed")?;

    info!(
        agent_id = %session.agent_id(),
        room = ?session.assigned_room(),
        "onboarding complete — assigned to room"
    );

    // ── Heartbeat loop (runs until interrupted) ─────────────────────
    loop {
        match session.heartbeat().await {
            Ok(status) => {
                info!(status = ?status, "heartbeat acknowledged");
            }
            Err(e) => {
                warn!(error = %e, "heartbeat failed — will retry");
            }
        }
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}
