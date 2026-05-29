//! # OpenConstruct — DAG Workflow (Rust)
//!
//! Defines a directed-acyclic-graph workflow of tasks and executes it
//! through the OpenConstruct workflow engine. Handles success, failure,
//! and retry semantics.
//!
//! ## Usage
//! ```sh
//! cargo run --example workflow_dag -- --coordinator ws://localhost:9142
//! ```

use anyhow::{Context, Result};
use clap::Parser;
use openconstruct::{
    workflow::{
        Dag, DagExecution, NodeStatus, TaskDef, TaskResult, WorkflowClient,
    },
    CoordinatorClient,
};
use std::time::Duration;
use tracing::info;

#[derive(Parser)]
#[command(name = "workflow_dag", about = "Define and execute a DAG workflow")]
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

    let workflows = WorkflowClient::new(&client);

    // ── Build the DAG ───────────────────────────────────────────────
    //
    //   fetch_sensors ──→ detect_anomalies ──→ notify
    //                  ╲                     ╱
    //                   ╲→ log_raw_data ────╱
    //
    let dag = Dag::new("anomaly-pipeline")
        .description("Detect anomalies in sensor data and alert")
        .task(
            TaskDef::new("fetch_sensors")
                .timeout(Duration::from_secs(30))
                .retries(2),
        )
        .task(TaskDef::new("detect_anomalies").timeout(Duration::from_secs(60)))
        .task(TaskDef::new("log_raw_data").timeout(Duration::from_secs(10)))
        .task(TaskDef::new("notify").timeout(Duration::from_secs(15)))
        .edge("fetch_sensors", "detect_anomalies")?
        .edge("fetch_sensors", "log_raw_data")?
        .edge("detect_anomalies", "notify")?
        .edge("log_raw_data", "notify")?;

    info!(dag = %dag.id(), "DAG defined with {} tasks", dag.task_count());

    // ── Submit the workflow ─────────────────────────────────────────
    let exec: DagExecution = workflows.submit(&dag).await?;
    info!(execution_id = %exec.id(), "workflow submitted");

    println!("Workflow: {}", dag.name());
    println!("Execution: {}", exec.id());
    println!("─────────────────────────────");

    // ── Watch execution progress ────────────────────────────────────
    let mut stream = exec.watch().await?;

    while let Some(update) = stream.next().await {
        let task_id = update.task_id();
        let status = update.status();
        let icon = match status {
            NodeStatus::Pending => "⏳",
            NodeStatus::Running => "🔄",
            NodeStatus::Succeeded => "✅",
            NodeStatus::Failed => "❌",
            NodeStatus::Skipped => "⏭️",
        };
        println!("  {} {} — {:?}", icon, task_id, status);
        info!(task = %task_id, ?status, "task update");

        if update.is_terminal() && update.all_terminal() {
            break;
        }
    }

    // ── Print final summary ─────────────────────────────────────────
    let result = workflows.result(&exec).await?;
    match result {
        TaskResult::Success(summary) => {
            println!("\n✅ Workflow completed in {:?}", summary.duration());
        }
        TaskResult::Partial(summary) => {
            println!(
                "\n⚠️ Workflow partially completed: {} of {} tasks succeeded",
                summary.succeeded(),
                summary.total()
            );
        }
        TaskResult::Failed(err) => {
            println!("\n❌ Workflow failed: {}", err.message());
        }
    }

    Ok(())
}
