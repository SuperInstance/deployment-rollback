# Deployment Rollback

**A deployment lifecycle management library** that models the state machine of software deployments — tracking running, rolled-back, and failed states — with automatic rollback triggers based on health-check failure conditions.

## Why It Matters

Modern deployment systems (Kubernetes rolling updates, Spinnaker, Argo CD) all face the same problem: how do you safely roll out a new version and, critically, how do you *undo* it when things go wrong? A deployment rollback reverts a service to its previous known-good version when health checks fail (error rate exceeds threshold, latency spikes, crash loops).

The key design decision is encoding deployment state as a type-safe enum: `Running`, `RolledBack`, `Failed`. The compiler enforces that only valid states exist, and the state machine ensures that a rolled-back deployment is clearly distinguishable from one that's still running or one that failed catastrophically.

**Real-world relevance:** This models the same pattern as Kubernetes Deployments (where rollback creates a new ReplicaSet with the previous version), blue-green deployments (switch traffic back to the idle environment), and canary deployments (stop routing traffic to the canary). The `rollback()` method records the reason — essential for post-incident reviews and deployment analytics.

## How It Works

The library models deployments as stateful records with three core fields:

**`Deployment` struct:**
- `id: String` — Unique deployment identifier (e.g., `dep-001`)
- `version: String` — Semantic version being deployed (e.g., `v2.3.1`)
- `state: DeployState` — Current lifecycle state
- `created_at: SystemTime` — When the deployment was initiated

**State machine:**
- `Running` → `RolledBack` (via `rollback(reason)`) — Health check failed, revert to previous version
- `Running` → `Failed` (implicit) — Deployment couldn't complete
- The `rollback` method is the only valid state transition, ensuring rollbacks are intentional and always carry a reason

**Rollback trigger:** The `rollback(&mut self, reason: &str)` method transitions the deployment to `RolledBack` and logs the reason. In production, this would be triggered by automated health checks (error rate > 5%, p99 latency > 500ms, crash-loop detection) rather than manual intervention.

## Quick Start

```rust
use deployment_rollback::{Deployment, DeployState};
use std::time::SystemTime;

let mut deployment = Deployment {
    id: "dep-001".into(),
    version: "v2.3.1".into(),
    state: DeployState::Running,
    created_at: SystemTime::now(),
};

// Simulate a failed health check → trigger rollback
let error_rate = 0.07; // 7% error rate
if error_rate > 0.05 {
    deployment.rollback("error rate exceeded 5% threshold");
}

assert!(matches!(deployment.state, DeployState::RolledBack));
```

## API

### `DeployState` (enum)
- `Running` — Deployment is live and accepting traffic
- `RolledBack` — Deployment was reverted to a previous version
- `Failed` — Deployment failed to complete

### `Deployment`
- `id: String` — Unique identifier
- `version: String` — Version string
- `state: DeployState` — Current state
- `created_at: SystemTime` — Creation timestamp
- `rollback(&mut self, reason: &str)` — Transition to `RolledBack` with reason. O(1)

## Architecture Notes

This library provides the deployment state model for SuperInstance's CI/CD pipeline. It integrates with the container runtime for version management and with the metrics forwarder for health-check-driven automated rollback.

See the full architecture: [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)

## License

MIT
