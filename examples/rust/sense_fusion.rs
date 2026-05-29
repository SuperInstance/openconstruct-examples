//! # OpenConstruct — Sense Fusion (Rust)
//!
//! Ingests vision and sonar shadow events from multiple sensors,
//! publishes them to the fusion pipeline, and prints the fused
//! occupancy grid as it updates.
//!
//! ## Usage
//! ```sh
//! cargo run --example sense_fusion -- --coordinator ws://localhost:9142
//! ```

use anyhow::{Context, Result};
use clap::Parser;
use openconstruct::{
    fusion::{FusionClient, FusionConfig, GridResolution},
    sense::{SensorReading, SensorType, ShadowEvent},
    CoordinatorClient,
};
use std::time::Duration;
use tracing::{info, warn};

#[derive(Parser)]
#[command(name = "sense_fusion", about = "Fuse vision + sonar shadow events")]
struct Args {
    #[arg(long, default_value = "ws://localhost:9142")]
    coordinator: String,

    /// Room to fuse sensors for.
    #[arg(long, default_value = "living-room")]
    room: String,
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

    // ── Set up the fusion pipeline ──────────────────────────────────
    let fusion = FusionClient::new(
        &client,
        FusionConfig {
            room: args.room.clone(),
            resolution: GridResolution::Cell10cm,
            confidence_threshold: 0.6,
            max_age: Duration::from_secs(30),
        },
    )
    .await
    .context("failed to create fusion client")?;

    info!(room = %args.room, "fusion pipeline started");

    // ── Subscribe to fused grid updates ─────────────────────────────
    let mut grid_stream = fusion.subscribe_grid().await?;

    // ── Publish some synthetic sensor readings ──────────────────────
    // In a real deployment these come from hardware drivers.
    let vision_reading = SensorReading::new(
        "cam-001",
        SensorType::Vision,
        ShadowEvent::detection(0.5, 1.2, 0.8, 0.95), // x, y, radius, confidence
    );

    let sonar_reading = SensorReading::new(
        "sonar-001",
        SensorType::Sonar,
        ShadowEvent::detection(0.55, 1.15, 0.7, 0.80),
    );

    fusion.publish(&vision_reading).await?;
    fusion.publish(&sonar_reading).await?;
    info!("published 2 synthetic sensor readings");

    // ── Print fused grid as it updates ──────────────────────────────
    println!("Fused Occupancy Grid (room: {})", args.room);
    println!("─────────────────────────────────");

    for _ in 0..5 {
        match tokio::time::timeout(Duration::from_secs(10), grid_stream.next()).await {
            Ok(Some(grid)) => {
                info!(
                    resolution = ?grid.resolution(),
                    occupied_cells = grid.occupied_count(),
                    "grid updated"
                );
                println!(
                    "Grid: {}×{} cells, {} occupied",
                    grid.width(),
                    grid.height(),
                    grid.occupied_count()
                );
            }
            Ok(None) => {
                warn!("grid stream ended");
                break;
            }
            Err(_) => {
                warn!("grid update timed out — no new sensor data?");
            }
        }
    }

    Ok(())
}
