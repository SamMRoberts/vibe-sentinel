use crate::domain::{ReadinessState, StatusCheck, StatusReport, VibeError};
use crate::ports::WorkspaceProbe;

pub struct StatusService<P: WorkspaceProbe> {
    probe: P,
}

impl<P: WorkspaceProbe> StatusService<P> {
    pub fn new(probe: P) -> Self {
        Self { probe }
    }

    pub fn evaluate(&self) -> Result<StatusReport, VibeError> {
        let harness_docs_present = self.probe.exists("AGENTS.md")?
            && self.probe.exists("docs/harness/scope.md")?
            && self.probe.exists("docs/harness/operating-model.md")?;
        let has_active_plan = self.probe.has_any_active_plan()?;
        let has_rust_workspace = self.probe.exists("Cargo.toml")?;

        Ok(StatusReport {
            project_name: "vibe-sentinel".to_string(),
            checks: vec![
                Self::check(
                    "harness docs",
                    harness_docs_present,
                    "required harness docs present",
                    "required harness docs missing",
                ),
                Self::check(
                    "active plan",
                    has_active_plan,
                    "active execution plan present",
                    "no active execution plan found",
                ),
                Self::check(
                    "rust workspace",
                    has_rust_workspace,
                    "Cargo workspace present",
                    "Cargo workspace missing",
                ),
            ],
        })
    }

    fn check(name: &str, ready: bool, ready_detail: &str, missing_detail: &str) -> StatusCheck {
        StatusCheck {
            name: name.to_string(),
            state: if ready {
                ReadinessState::Ready
            } else {
                ReadinessState::Missing
            },
            detail: if ready { ready_detail } else { missing_detail }.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StatusService;
    use crate::adapters::test_support::FakeWorkspaceProbe;
    use crate::domain::ReadinessState;

    #[test]
    fn status_service_reports_missing_checks_by_default() {
        let service = StatusService::new(FakeWorkspaceProbe::new());

        let report = service.evaluate().expect("status report");

        assert_eq!(report.project_name, "vibe-sentinel");
        assert_eq!(report.check_count(), 3);
        assert!(report
            .checks
            .iter()
            .all(|check| check.state == ReadinessState::Missing));
    }

    #[test]
    fn status_service_uses_workspace_probe() {
        let service = StatusService::new(
            FakeWorkspaceProbe::new()
                .with_path("AGENTS.md")
                .with_path("docs/harness/scope.md")
                .with_path("docs/harness/operating-model.md")
                .with_active_plan(true),
        );

        let report = service.evaluate().expect("status report");

        assert_eq!(report.project_name, "vibe-sentinel");
        assert_eq!(report.check_count(), 3);
        assert_eq!(report.checks[0].name, "harness docs");
        assert_eq!(report.checks[0].state, ReadinessState::Ready);
        assert_eq!(report.checks[1].name, "active plan");
        assert_eq!(report.checks[1].state, ReadinessState::Ready);
        assert_eq!(report.checks[2].name, "rust workspace");
        assert_eq!(report.checks[2].state, ReadinessState::Missing);
        assert!(!report.is_ready());
    }
}
