//! # OpenConstruct — Policy Check (Rust)
//!
//! Demonstrates how to set up access policies and evaluate requests
//! against them. Shows allow/deny decisions with audit logging.
//!
//! ## Usage
//! ```sh
//! cargo run --example policy_check -- --coordinator ws://localhost:9142
//! ```

use anyhow::{Context, Result};
use clap::Parser;
use openconstruct::{
    policy::{Action, Policy, PolicyClient, PolicyEffect, Request, Resource, Subject},
    CoordinatorClient,
};
use tracing::info;

#[derive(Parser)]
#[command(name = "policy_check", about = "Evaluate access policies")]
struct Args {
    #[arg(long, default_value = "ws://localhost:9142")]
    coordinator: String,
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

    let policy = PolicyClient::new(&client);

    // ── Define policies ─────────────────────────────────────────────
    let allow_sensor_read = Policy::new("allow-sensor-read")
        .description("Agents may read sensor data from their assigned room")
        .effect(PolicyEffect::Allow)
        .subject(Subject::role("agent"))
        .action(Action::Read)
        .resource(Resource::pattern("sensor/*/data"));

    let deny_actuator_override = Policy::new("deny-actuator-override")
        .description("Agents may not override actuator safety limits")
        .effect(PolicyEffect::Deny)
        .subject(Subject::any())
        .action(Action::Write)
        .resource(Resource::pattern("actuator/*/safety"));

    let admin_full = Policy::new("admin-full-access")
        .description("Admins have full access to everything")
        .effect(PolicyEffect::Allow)
        .subject(Subject::role("admin"))
        .action(Action::All)
        .resource(Resource::all());

    // ── Upload policies ─────────────────────────────────────────────
    for p in [&allow_sensor_read, &deny_actuator_override, &admin_full] {
        policy.upsert(p).await?;
        info!(policy = %p.id(), "policy uploaded");
    }

    // ── Evaluate sample requests ────────────────────────────────────
    let requests = vec![
        (
            "agent reads own sensor",
            Request::new(Subject::role("agent"), Action::Read, Resource::exact("sensor/cam-001/data")),
        ),
        (
            "agent tries to override safety",
            Request::new(Subject::role("agent"), Action::Write, Resource::exact("actuator/door-001/safety")),
        ),
        (
            "admin overrides safety",
            Request::new(Subject::role("admin"), Action::Write, Resource::exact("actuator/door-001/safety")),
        ),
    ];

    println!("Policy Evaluation Results");
    println!("══════════════════════════");

    for (label, req) in &requests {
        let decision = policy.evaluate(req).await?;
        let icon = match decision.effect() {
            PolicyEffect::Allow => "✅",
            PolicyEffect::Deny => "🚫",
        };
        println!("{} {} → {} ({})", icon, label, decision.effect(), decision.reason());
        info!(label, effect = ?decision.effect(), reason = %decision.reason(), "evaluated");
    }

    Ok(())
}
