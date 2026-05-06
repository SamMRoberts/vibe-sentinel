use std::fmt::{Display, Formatter};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadinessState {
    Ready,
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusCheck {
    pub name: String,
    pub state: ReadinessState,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::{ReadinessState, StatusCheck, StatusReport};

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
}
