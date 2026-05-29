/**
 * # OpenConstruct — Agent Onboarding (C / FFI)
 *
 * Demonstrates the C ABI for OpenConstruct. Uses the shared library
 * (`libopenconstruct.so` / `openconstruct.dll`) to onboard an agent
 * and run a simple heartbeat loop.
 *
 * ## Build
 *   cc onboard.c -lopenconstruct -o onboard
 *
 * ## Run
 *   ./onboard ws://localhost:9142 my-c-agent
 */

#include <openconstruct.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

/* ── Globals for graceful shutdown ────────────────────────────────── */
static volatile int g_running = 1;

static void handle_signal(int sig) {
    (void)sig;
    g_running = 0;
}

/* ── Helpers ──────────────────────────────────────────────────────── */
static void log_error(const char *prefix, OCError *err) {
    if (err) {
        fprintf(stderr, "[ERROR] %s: %s (code %d)\n", prefix,
                oc_error_message(err), oc_error_code(err));
        oc_error_free(err);
    } else {
        fprintf(stderr, "[ERROR] %s: unknown error\n", prefix);
    }
}

/* ── Main ─────────────────────────────────────────────────────────── */
int main(int argc, char **argv) {
    const char *coordinator = (argc > 1) ? argv[1] : "ws://localhost:9142";
    const char *agent_name  = (argc > 2) ? argv[2] : "c-agent";

    signal(SIGINT,  handle_signal);
    signal(SIGTERM, handle_signal);

    /* ── Connect ──────────────────────────────────────────────────── */
    OCError *err = NULL;
    OCClient *client = oc_client_connect(coordinator, &err);
    if (!client) {
        log_error("failed to connect", err);
        return 1;
    }
    printf("[INFO] connected to %s\n", coordinator);

    /* ── Declare identity ─────────────────────────────────────────── */
    OCCapability caps[2] = { OC_CAP_SENSE, OC_CAP_ACT };

    OCAgentIdentity identity = {
        .name         = agent_name,
        .capabilities = caps,
        .cap_count    = 2,
        .metadata     = "{\"language\":\"c\",\"example\":\"onboard\"}",
    };

    /* ── Onboard ──────────────────────────────────────────────────── */
    OCSession *session = oc_client_onboard(client, &identity, &err);
    if (!session) {
        log_error("onboarding failed", err);
        oc_client_free(client);
        return 1;
    }

    printf("[INFO] onboarded as %s — assigned room: %s\n",
           oc_session_agent_id(session),
           oc_session_room(session));

    /* ── Heartbeat loop ───────────────────────────────────────────── */
    while (g_running) {
        OCHeartbeatStatus status;
        int rc = oc_session_heartbeat(session, &status, &err);
        if (rc != 0) {
            fprintf(stderr, "[WARN] heartbeat failed: %s\n",
                    err ? oc_error_message(err) : "unknown");
            if (err) oc_error_free(err);
            err = NULL;
        } else {
            printf("[INFO] heartbeat ok — connected_agents: %u\n",
                   status.connected_agents);
        }
        sleep(30);
    }

    /* ── Cleanup ──────────────────────────────────────────────────── */
    oc_session_disconnect(session);
    oc_session_free(session);
    oc_client_free(client);

    printf("[INFO] disconnected — bye\n");
    return 0;
}
