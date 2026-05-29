/**
 * # OpenConstruct — Agent Onboarding (TypeScript)
 *
 * Registers an agent with the OpenConstruct coordinator and maintains
 * a heartbeat loop. Uses the official `@openconstruct/sdk` package.
 *
 * ## Usage
 *   npm install @openconstruct/sdk
 *   npx ts-node onboard.ts -- --coordinator ws://localhost:9142
 */

import {
  CoordinatorClient,
  AgentIdentity,
  Capability,
  type Session,
} from "@openconstruct/sdk";

const DEFAULT_COORDINATOR = "ws://localhost:9142";
const DEFAULT_NAME = "typescript-agent";
const HEARTBEAT_INTERVAL_MS = 30_000;

// ── Parse CLI args minimally ────────────────────────────────────────
function parseArgs(): { coordinator: string; name: string } {
  const args = process.argv.slice(2);
  let coordinator = DEFAULT_COORDINATOR;
  let name = DEFAULT_NAME;

  for (let i = 0; i < args.length; i++) {
    if (args[i] === "--coordinator" && args[i + 1]) {
      coordinator = args[++i];
    } else if (args[i] === "--name" && args[i + 1]) {
      name = args[++i];
    }
  }

  return { coordinator, name };
}

// ── Heartbeat loop ──────────────────────────────────────────────────
async function heartbeatLoop(session: Session): Promise<void> {
  const interval = setInterval(async () => {
    try {
      const status = await session.heartbeat();
      console.log(`[heartbeat] ok — ${JSON.stringify(status)}`);
    } catch (err) {
      console.warn(`[heartbeat] failed: ${err}`);
    }
  }, HEARTBEAT_INTERVAL_MS);

  // Clean up on SIGINT
  process.on("SIGINT", () => {
    clearInterval(interval);
    session.disconnect();
    console.log("disconnected — bye");
    process.exit(0);
  });
}

// ── Main ────────────────────────────────────────────────────────────
async function main(): Promise<void> {
  const { coordinator, name } = parseArgs();

  // Connect
  const client = new CoordinatorClient(coordinator);
  await client.connect();
  console.log(`connected to ${coordinator}`);

  // Declare identity
  const identity: AgentIdentity = {
    name,
    capabilities: [Capability.Sense, Capability.Act],
    metadata: {
      language: "typescript",
      example: "onboard",
    },
  };

  // Onboard
  const session = await client.onboard(identity);
  console.log(
    `onboarded as ${session.agentId} — assigned to room ${session.assignedRoom}`
  );

  // Heartbeat loop
  await heartbeatLoop(session);
}

main().catch((err) => {
  console.error("fatal:", err);
  process.exit(1);
});
