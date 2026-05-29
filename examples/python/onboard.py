#!/usr/bin/env python3
"""
# OpenConstruct — Agent Onboarding (Python)

Thin-client example that registers an agent with the OpenConstruct
coordinator and enters a heartbeat loop using the official Python SDK.

## Usage
    pip install openconstruct
    python onboard.py --coordinator ws://localhost:9142
"""

from __future__ import annotations

import argparse
import logging
import signal
import sys
import time

from openconstruct import AgentIdentity, Capability, CoordinatorClient

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
)
log = logging.getLogger("onboard")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="OpenConstruct Python onboarding example")
    parser.add_argument(
        "--coordinator",
        default="ws://localhost:9142",
        help="Coordinator WebSocket URL",
    )
    parser.add_argument("--name", default="python-agent", help="Agent name")
    return parser.parse_args()


def main() -> None:
    args = parse_args()

    # ── Connect to the coordinator ───────────────────────────────────
    client = CoordinatorClient.connect(args.coordinator)
    log.info("connected to %s", args.coordinator)

    # ── Declare identity and capabilities ────────────────────────────
    identity = AgentIdentity(
        name=args.name,
        capabilities=[Capability.SENSE, Capability.ACT],
        metadata={
            "language": "python",
            "example": "onboard",
        },
    )

    # ── Onboard ──────────────────────────────────────────────────────
    session = client.onboard(identity)
    log.info(
        "onboarded as %s — assigned to room %s",
        session.agent_id,
        session.assigned_room,
    )

    # ── Graceful shutdown on SIGINT ──────────────────────────────────
    running = True

    def _shutdown(signum: int, _frame: object) -> None:
        nonlocal running
        log.info("received signal %d, shutting down", signum)
        running = False

    signal.signal(signal.SIGINT, _shutdown)
    signal.signal(signal.SIGTERM, _shutdown)

    # ── Heartbeat loop ───────────────────────────────────────────────
    while running:
        try:
            status = session.heartbeat()
            log.info("heartbeat ok — %s", status)
        except Exception as exc:
            log.warning("heartbeat failed: %s", exc)

        for _ in range(30):  # 30 × 1s = 30s sleep, interruptible
            if not running:
                break
            time.sleep(1)

    # ── Clean disconnect ─────────────────────────────────────────────
    session.disconnect()
    log.info("disconnected — bye")
    sys.exit(0)


if __name__ == "__main__":
    main()
