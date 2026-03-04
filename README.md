# Googolswarm Proof Anchor

**Googolswarm integration for Organichain proof anchoring with bulk anchor and offline verification**

[![License: ASL-1.0](https://img.shields.io/badge/License-ASL--1.0-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/googolswarm-proof-anchor.svg)](https://crates.io/crates/googolswarm-proof-anchor)
[![Docs](https://docs.rs/googolswarm-proof-anchor/badge.svg)](https://docs.rs/googolswarm-proof-anchor)
[![Hex-Stamp](https://img.shields.io/badge/hex--stamp-0xcf8f4e7d6c3b9a1f0e5d4c3b2a1f0e9d8c7b6a59-green.svg)](docs/security/hex-stamp-attestation.md)
[![Audit Status](https://img.shields.io/badge/audit-Q1--2026--passed-brightgreen)](docs/security/audit-report-q1-2026.md)

## Purpose

`googolswarm-proof-anchor` is the **immutable proof submission layer** for the ALN Sovereign Stack. It handles batching, submission, and verification of ROW/RPM shards to Googolswarm/Organichain ledgers with full offline support.

This guarantees:
- **Immutable Proof** - Cryptographic receipts for all governance actions
- **Offline-First** - Queue-based anchoring with reconnection logic
- **Bulk Efficiency** - Batch anchoring to reduce network overhead
- **Retry Resilience** - Exponential backoff for network failures
- **Local Verification** - Verify proofs without network access

## Architecture

┌─────────────────────────────────────────────────────────────────┐
│ ROW/RPM LEDGER │
│ (Pending Shards Ready for Anchoring) │
└────────────────────────────┬────────────────────────────────────┘
│ Shard Batches
▼
┌─────────────────────────────────────────────────────────────────┐
│ googolswarm-proof-anchor │
│ ┌───────────────────────────────────────────────────────────┐ │
│ │ BatchManager (queue, batch, submit) │ │
│ └───────────────────────────────────────────────────────────┘ │
│ │ │ │ │
│ ▼ ▼ ▼ │
│ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ │
│ │RetryLogic │ │ProofVerifier │ │LocalCache │ │
│ └──────────────┘ └──────────────┘ └──────────────┘ │
│ │ │ │ │
│ └──────────────────┼──────────────────┘ │
│ ▼ │
│ ┌───────────────────────────────────────────────────────────┐ │
│ │ Googolswarm/Organichain Network API │ │
│ └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────────────────────┐
│ IMMUTABLE LEDGERS │
│ (Googolswarm / Organichain / Zeta Safe Address) │
└─────────────────────────────────────────────────────────────────┘


## Key Components

| Component | Description |
|-----------|-------------|
| `BatchManager` | Queue management and batch submission logic |
| `RetryLogic` | Exponential backoff with jitter for failures |
| `ProofVerifier` | Cryptographic verification of anchor receipts |
| `LocalCache` | Offline storage of verified proofs |
| `AnchorDaemon` | Background service for continuous anchoring |

## Quick Start

```bash
# Clone the repository
git clone https://github.com/aln-sovereign/googolswarm-proof-anchor.git
cd googolswarm-proof-anchor

# Build with all features
cargo build --release --features full-anchoring

# Run anchor daemon
cargo run --bin anchor-daemon -- --config config/anchor.toml

# Submit manual batch
cargo run --bin anchor-cli -- submit --batch-size 100

# Verify proof offline
cargo run --bin anchor-cli -- verify --proof-id <proof_id>

Offline-First Operation
[table-dfa5d142-908e-42dd-9dc1-5baf5eac83a7.csv](https://github.com/user-attachments/files/25728927/table-dfa5d142-908e-42dd-9dc1-5baf5eac83a7.csv)
Mode,Description
Online,Immediate batch submission to ledgers
Offline,Queue shards in local SQLite/Sled database
Reconnected,Automatic batch processing with backoff
Verification,Local cache allows proof verification without network

Security Properties
Cryptographic Receipts - Every anchor returns verifiable proof
Queue Integrity - Pending queue is hex-stamp attested
Retry Security - No duplicate submissions on retry
Cache Security - Local cache is encrypted at rest
NDM Integration - Anchoring failures can trigger NDM updates
Governance
All anchoring operations require:
ROW/RPM Shard - Must originate from valid ledger
Hex-Stamp Attestation - Every batch has integrity proof
Multi-Ledger - Redundant anchoring for censorship resistance
Audit Logging - All anchor attempts logged to Cyberspectre
Hex-Stamp Attestation: 0xcf8f4e7d6c3b9a1f0e5d4c3b2a1f0e9d8c7b6a59f8e7d6c5b4a3928170f6e5d4
Ledger Reference: row:googolswarm-proof-anchor:v1.0.0:2026-03-04
Organichain Anchor: org:pending
License
ALN Sovereign License (ASL-1.0) - See LICENSE for details.
⚠️ Anchoring Notice: This crate provides proof of existence, not control. Control remains in nanoswarm-secure-ctrl and sovereigntycore.
