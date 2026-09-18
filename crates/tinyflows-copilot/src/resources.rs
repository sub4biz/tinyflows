// SPDX-License-Identifier: GPL-3.0-or-later
//! Reference material for authors of tinyflows graphs.
//!
//! Hosts decide how these bytes are exposed to an agent.  The content itself
//! is portable: it documents the graph language and deliberately avoids a
//! dependency on a particular harness or skill runtime.

/// One file in a portable authoring resource bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceFile {
    /// Slash-separated path relative to the bundle root.
    pub path: &'static str,
    /// UTF-8 resource contents.
    pub contents: &'static str,
}

/// The flow-authoring manual's manifest page.
pub const FLOW_AUTHORING_WORKFLOW: &str = include_str!("resources/flow-authoring/WORKFLOW.md");
/// Expression and jq reference page.
pub const FLOW_AUTHORING_EXPRESSIONS: &str =
    include_str!("resources/flow-authoring/references/expressions.md");
/// Node-configuration reference page.
pub const FLOW_AUTHORING_NODE_CONFIG: &str =
    include_str!("resources/flow-authoring/references/node-config.md");
/// Dry-run interpretation reference page.
pub const FLOW_AUTHORING_DRY_RUN: &str =
    include_str!("resources/flow-authoring/references/dry-run.md");

/// Every file in the portable `flow-authoring` manual.
pub const FLOW_AUTHORING_FILES: &[ResourceFile] = &[
    ResourceFile {
        path: "WORKFLOW.md",
        contents: FLOW_AUTHORING_WORKFLOW,
    },
    ResourceFile {
        path: "references/expressions.md",
        contents: FLOW_AUTHORING_EXPRESSIONS,
    },
    ResourceFile {
        path: "references/node-config.md",
        contents: FLOW_AUTHORING_NODE_CONFIG,
    },
    ResourceFile {
        path: "references/dry-run.md",
        contents: FLOW_AUTHORING_DRY_RUN,
    },
];

#[cfg(test)]
#[path = "resources_tests.rs"]
mod tests;
