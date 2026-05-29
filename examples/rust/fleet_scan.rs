//! # OpenConstruct — Fleet Scan (Rust)
//!
//! Discovers all nodes in the fleet via the coordinator and prints a
//! text-based topology tree. Useful for debugging and monitoring.
//!
//! ## Usage
//! ```sh
//! cargo run --example fleet_scan -- --coordinator ws://localhost:9142
//! ```

use anyhow::{Context, Result};
use clap::Parser;
use openconstruct::{CoordinatorClient, FleetFilter};
use tracing::info;

#[derive(Parser)]
#[command(name = "fleet_scan", about = "Discover fleet topology")]
struct Args {
    #[arg(long, default_value = "ws://localhost:9142")]
    coordinator: String,

    /// Only show nodes in the given room.
    #[arg(long)]
    room: Option<String>,

    /// Show capabilities for each node.
    #[arg(long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("openconstruct=debug,info")
        .init();

    let args = Args::parse();

    let client = CoordinatorClient::connect(&args.coordinator)
        .await
        .context("failed to connect to coordinator")?;

    let filter = FleetFilter {
        room: args.room.clone(),
        ..Default::default()
    };

    let topology = client
        .fleet_scan(filter)
        .await
        .context("fleet scan request failed")?;

    info!(node_count = topology.nodes().len(), "fleet scan complete");

    // ── Pretty-print the topology ───────────────────────────────────
    println!("OpenConstruct Fleet Topology");
    println!("═════════════════════════════");

    if topology.rooms().is_empty() {
        println!("  (no rooms discovered)");
        return Ok(());
    }

    for room in topology.rooms() {
        println!("🏢 {}", room.name());
        for node in topology.nodes_in_room(room.id()) {
            let caps = if args.verbose {
                let c: Vec<&str> = node.capabilities().iter().map(|c| c.as_str()).collect();
                format!(" [{}]", c.join(", "))
            } else {
                String::new()
            };
            println!("  ├─ {} ({}){}", node.name(), node.id(), caps);
        }
    }

    println!();
    println!(
        "Total: {} nodes across {} rooms",
        topology.nodes().len(),
        topology.rooms().len()
    );

    Ok(())
}
