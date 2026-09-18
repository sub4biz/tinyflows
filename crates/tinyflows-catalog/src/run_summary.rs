// SPDX-License-Identifier: GPL-3.0-or-later
//! Portable settlement rules for saved workflow run history.

use serde_json::Value;

use crate::FlowRunStep;

/// Reconstructs lightweight steps from an engine run output.
pub fn reconstruct_steps(output: &Value) -> Vec<FlowRunStep> {
    output
        .get("nodes")
        .and_then(Value::as_object)
        .map(|nodes| {
            nodes
                .iter()
                .map(|(node_id, slot)| FlowRunStep {
                    node_id: node_id.clone(),
                    output: slot.get("items").cloned().unwrap_or(Value::Null),
                    port: slot.get("port").and_then(Value::as_str).map(str::to_string),
                    ..Default::default()
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Merges observed steps with reconstructed output, retaining richer observed
/// timing, status, and diagnostic fields for matching nodes.
pub fn settle_steps(observed: Vec<FlowRunStep>, output: &Value) -> Vec<FlowRunStep> {
    let reconstructed = reconstruct_steps(output);
    if observed.is_empty() {
        return reconstructed;
    }
    let mut settled = observed;
    for step in reconstructed {
        if let Some(existing) = settled
            .iter_mut()
            .find(|existing| existing.node_id == step.node_id)
        {
            // The live observer has richer timing/status data, but only the
            // post-hoc output knows which branch a routing node selected.
            if existing.port.is_none() {
                existing.port = step.port;
            }
        } else {
            settled.push(step);
        }
    }
    settled
}

/// A terminal run classification suitable for persisting in [`crate::FlowRun`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalRunStatus {
    /// Persisted status string.
    pub status: &'static str,
    /// Error text when a continued/routed step failed.
    pub error: Option<String>,
}

/// Classifies settled steps, with a pending approval taking precedence.
pub fn terminal_status(steps: &[FlowRunStep], pending_approvals: &[String]) -> TerminalRunStatus {
    if !pending_approvals.is_empty() {
        return TerminalRunStatus {
            status: "pending_approval",
            error: None,
        };
    }
    let failed: Vec<&str> = steps
        .iter()
        .filter(|step| step.status.as_deref() == Some("error"))
        .map(|step| step.node_id.as_str())
        .collect();
    if !failed.is_empty() {
        return TerminalRunStatus {
            status: "failed",
            error: Some(format!(
                "node(s) failed after retries: {}",
                failed.join(", ")
            )),
        };
    }
    if steps.iter().any(|step| !step.diagnostics.is_empty()) {
        TerminalRunStatus {
            status: "completed_with_warnings",
            error: None,
        }
    } else {
        TerminalRunStatus {
            status: "completed",
            error: None,
        }
    }
}
