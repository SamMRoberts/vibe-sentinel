use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VibeError {
    InvalidArguments(String),
    WorkspaceUnreadable(String),
    StatusEvaluationFailed(String),
}

impl Display for VibeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidArguments(message)
            | Self::WorkspaceUnreadable(message)
            | Self::StatusEvaluationFailed(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for VibeError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessState {
    Ready,
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCheck {
    pub name: String,
    pub state: ReadinessState,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusReport {
    pub project_name: String,
    pub checks: Vec<StatusCheck>,
}

impl StatusReport {
    pub fn is_ready(&self) -> bool {
        self.checks
            .iter()
            .all(|check| check.state == ReadinessState::Ready)
    }

    pub fn check_count(&self) -> usize {
        self.checks.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSeverity {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationState {
    Ready,
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidationEvidence {
    pub section: String,
    pub line: Option<usize>,
    pub excerpt: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidationIssue {
    pub rule_id: String,
    pub severity: ValidationSeverity,
    pub message: String,
    pub evidence: Option<ValidationEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidationCheck {
    pub rule_id: String,
    pub state: ValidationState,
    pub severity: ValidationSeverity,
    pub message: String,
    pub evidence: Option<ValidationEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlanValidationReport {
    pub path: String,
    pub ready: bool,
    pub checks: Vec<ValidationCheck>,
    pub issues: Vec<ValidationIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ActivePlansValidationReport {
    pub project_name: String,
    pub ready: bool,
    pub plans: Vec<PlanValidationReport>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TddGateAction {
    StartArchitecture,
    StartSkeletons,
    StartMockTests,
    StartImplementation,
    CompletePlan,
}

impl TddGateAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::StartArchitecture => "start_architecture",
            Self::StartSkeletons => "start_skeletons",
            Self::StartMockTests => "start_mock_tests",
            Self::StartImplementation => "start_implementation",
            Self::CompletePlan => "complete_plan",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TddWorkflowPhase {
    Idle,
    PlanCreated,
    PlanReviewed,
    ArchitectureReviewed,
    ImplementationReady,
    ImplementationUnderway,
    CompleteReady,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TddGateReport {
    pub project_name: String,
    pub allowed: bool,
    pub current_phase: TddWorkflowPhase,
    pub blocking_issues: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
    pub next_allowed_actions: Vec<TddGateAction>,
}

#[cfg(test)]
mod tests {
    use super::{ReadinessState, StatusCheck, StatusReport, TddGateAction, TddWorkflowPhase};

    #[test]
    fn status_report_readiness_reflects_checks() {
        let report = StatusReport {
            project_name: "vibe-sentinel".to_string(),
            checks: vec![StatusCheck {
                name: "harness docs".to_string(),
                state: ReadinessState::Ready,
                detail: "present".to_string(),
            }],
        };

        assert!(report.is_ready());
        assert_eq!(report.check_count(), 1);
    }

    #[test]
    fn tdd_gate_action_serializes_as_mcp_argument_value() {
        let value =
            serde_json::to_value(TddGateAction::StartImplementation).expect("serialize action");

        assert_eq!(value, serde_json::json!("start_implementation"));
        assert_eq!(
            TddGateAction::StartImplementation.as_str(),
            "start_implementation"
        );
    }

    #[test]
    fn tdd_workflow_phase_serializes_as_structured_content_value() {
        let value = serde_json::to_value(TddWorkflowPhase::PlanReviewed).expect("serialize phase");

        assert_eq!(value, serde_json::json!("plan_reviewed"));
    }
}
