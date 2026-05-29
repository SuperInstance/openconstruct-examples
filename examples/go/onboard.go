// Package main implements a basic OpenConstruct agent onboarding example in Go.
//
// # OpenConstruct — Agent Onboarding (Go)
//
// Registers an agent with the OpenConstruct coordinator and maintains
// a heartbeat loop using the official Go SDK.
//
// ## Usage
//
//	go run onboard.go --coordinator ws://localhost:9142
package main

import (
	"context"
	"flag"
	"log/slog"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/openconstruct/go-sdk"
)

func main() {
	// ── Flags ────────────────────────────────────────────────────────
	coordinator := flag.String("coordinator", "ws://localhost:9142", "Coordinator WebSocket URL")
	name := flag.String("name", "go-agent", "Agent name")
	flag.Parse()

	// ── Structured logger ────────────────────────────────────────────
	logger := slog.New(slog.NewTextHandler(os.Stdout, &slog.HandlerOptions{Level: slog.LevelInfo}))
	slog.SetDefault(logger)

	// ── Connect ──────────────────────────────────────────────────────
	client, err := openconstruct.Connect(*coordinator)
	if err != nil {
		slog.Error("failed to connect to coordinator", "error", err)
		os.Exit(1)
	}
	slog.Info("connected to coordinator", "url", *coordinator)

	// ── Declare identity and capabilities ────────────────────────────
	identity := openconstruct.AgentIdentity{
		Name:         *name,
		Capabilities: []openconstruct.Capability{openconstruct.CapSense, openconstruct.CapAct},
		Metadata: map[string]string{
			"language": "go",
			"example":  "onboard",
		},
	}

	// ── Onboard ──────────────────────────────────────────────────────
	session, err := client.Onboard(context.Background(), identity)
	if err != nil {
		slog.Error("onboarding failed", "error", err)
		os.Exit(1)
	}
	slog.Info("onboarded",
		"agent_id", session.AgentID(),
		"room", session.AssignedRoom(),
	)

	// ── Graceful shutdown ────────────────────────────────────────────
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	sigCh := make(chan os.Signal, 1)
	signal.Notify(sigCh, syscall.SIGINT, syscall.SIGTERM)

	go func() {
		sig := <-sigCh
		slog.Info("received signal, shutting down", "signal", sig)
		cancel()
	}()

	// ── Heartbeat loop ───────────────────────────────────────────────
	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			session.Disconnect()
			slog.Info("disconnected — bye")
			return
		case <-ticker.C:
			status, err := session.Heartbeat(ctx)
			if err != nil {
				slog.Warn("heartbeat failed", "error", err)
				continue
			}
			slog.Info("heartbeat acknowledged", "status", status)
		}
	}
}
