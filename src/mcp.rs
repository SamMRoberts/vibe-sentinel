use std::path::PathBuf;

use serde::Serialize;

use crate::core::StatusService;
use crate::domain::{StatusCheck, StatusReport, VibeError};
use crate::ports::WorkspaceProbe;

pub const STATUS_TOOL_NAME: &str = "vibe_sentinel_status";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McpServerConfig {
    pub root: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McpToolDescriptor {
    pub name: String,
    pub description: String,
    pub read_only: bool,
    pub idempotent: bool,
    pub local_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum McpStatusRequest {
    Status,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct McpStatusResponse {
    pub project_name: String,
    pub ready: bool,
    pub checks: Vec<StatusCheck>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum McpErrorCode {
    InvalidRequest,
    WorkspaceUnreadable,
    InternalError,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct McpErrorResponse {
    pub code: McpErrorCode,
    pub message: String,
}

pub fn status_tool_descriptor() -> McpToolDescriptor {
    McpToolDescriptor {
        name: STATUS_TOOL_NAME.to_string(),
        description: "Read local vibe-sentinel harness readiness status".to_string(),
        read_only: true,
        idempotent: true,
        local_only: true,
    }
}

pub fn evaluate_status_tool<P: WorkspaceProbe>(probe: P) -> Result<McpStatusResponse, VibeError> {
    StatusService::new(probe)
        .evaluate()
        .map(response_from_report)
}

pub fn response_from_report(report: StatusReport) -> McpStatusResponse {
    let ready = report.is_ready();
    McpStatusResponse {
        project_name: report.project_name,
        ready,
        checks: report.checks,
    }
}

pub fn map_error(error: VibeError) -> McpErrorResponse {
    let code = match error {
        VibeError::InvalidArguments(_) => McpErrorCode::InvalidRequest,
        VibeError::WorkspaceUnreadable(_) => McpErrorCode::WorkspaceUnreadable,
        VibeError::StatusEvaluationFailed(_) => McpErrorCode::InternalError,
    };

    McpErrorResponse {
        code,
        message: error.to_string(),
    }
}

pub fn run_stdio_server(config: McpServerConfig) -> Result<(), VibeError> {
    let _config = config;
    Err(VibeError::StatusEvaluationFailed(
        "MCP stdio server skeleton is not implemented yet".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{
        evaluate_status_tool, map_error, run_stdio_server, status_tool_descriptor, McpErrorCode,
        McpServerConfig, STATUS_TOOL_NAME,
    };
    use crate::adapters::test_support::FakeWorkspaceProbe;
    use crate::domain::ReadinessState;
    use crate::domain::VibeError;

    #[test]
    fn mcp_skeleton_exposes_status_tool_name() {
        assert_eq!(STATUS_TOOL_NAME, "vibe_sentinel_status");
    }

    #[test]
    fn status_tool_descriptor_is_read_only_idempotent_and_local() {
        let descriptor = status_tool_descriptor();

        assert_eq!(descriptor.name, STATUS_TOOL_NAME);
        assert!(descriptor.read_only);
        assert!(descriptor.idempotent);
        assert!(descriptor.local_only);
        assert!(descriptor.description.contains("readiness"));
    }

    #[test]
    fn status_tool_response_matches_status_report_shape() {
        let response = evaluate_status_tool(
            FakeWorkspaceProbe::new()
                .with_path("AGENTS.md")
                .with_path("docs/harness/scope.md")
                .with_path("docs/harness/operating-model.md")
                .with_path("Cargo.toml")
                .with_active_plan(true),
        )
        .expect("status response");

        assert_eq!(response.project_name, "vibe-sentinel");
        assert!(response.ready);
        assert_eq!(response.checks.len(), 3);
        assert!(response
            .checks
            .iter()
            .all(|check| check.state == ReadinessState::Ready));
    }

    #[test]
    fn status_tool_maps_workspace_errors_to_mcp_errors() {
        let response = map_error(VibeError::WorkspaceUnreadable(
            "could not read active plan directory".to_string(),
        ));

        assert_eq!(response.code, McpErrorCode::WorkspaceUnreadable);
        assert_eq!(response.message, "could not read active plan directory");
    }

    #[test]
    fn mcp_stdio_server_skeleton_reports_not_implemented() {
        let result = run_stdio_server(McpServerConfig {
            root: PathBuf::from("."),
        });

        assert_eq!(
            result,
            Err(VibeError::StatusEvaluationFailed(
                "MCP stdio server skeleton is not implemented yet".to_string()
            ))
        );
    }
}
