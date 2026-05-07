use crate::domain::{
    ActivePlanResource, ActivePlanResourceRead, ActivePlansValidationReport, PlanValidationReport,
    ReadinessState, StatusCheck, StatusReport, TddGateAction, TddGateReport, TddWorkflowPhase,
    ValidationCheck, ValidationEvidence, ValidationIssue, ValidationSeverity, ValidationState,
    VibeError,
};
use crate::ports::WorkspaceProbe;

pub const ACTIVE_PLAN_RESOURCE_URI_PREFIX: &str = "vibe-sentinel://active-plans/";

pub struct StatusService<P: WorkspaceProbe> {
    probe: P,
}

pub struct PlanValidationService<P: WorkspaceProbe> {
    probe: P,
}

pub struct TddGateService<P: WorkspaceProbe> {
    probe: P,
}

pub struct ActivePlanResourceService<P: WorkspaceProbe> {
    probe: P,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ActivePlanDocument {
    path: String,
    title: String,
    reviewed_plan_status: Option<SectionStatus>,
    reviewed_architecture_status: Option<SectionStatus>,
    skeleton_items: Vec<ChecklistItem>,
    mock_test_items: Vec<ChecklistItem>,
    implementation_items: Vec<ImplementationItem>,
    validation_log_entries: Vec<ValidationLogEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SectionStatus {
    value: String,
    line: usize,
    excerpt: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ChecklistItem {
    text: String,
    checked: bool,
    line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ImplementationItem {
    text: String,
    checked: bool,
    line: usize,
    validation_after_text: Option<String>,
    validation_after_line: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValidationLogEntry {
    text: String,
    line: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlanSection {
    ReviewedPlan,
    ReviewedArchitecture,
    SkeletonChecklist,
    MockTestChecklist,
    ImplementationChecklist,
    ValidationLog,
    Other,
}

impl<P: WorkspaceProbe> ActivePlanResourceService<P> {
    pub fn new(probe: P) -> Self {
        Self { probe }
    }

    pub fn list_resources(&self) -> Result<Vec<ActivePlanResource>, VibeError> {
        let _ = self.probe.active_plan_paths()?;
        Ok(Vec::new())
    }

    pub fn read_resource(&self, uri: &str) -> Result<ActivePlanResourceRead, VibeError> {
        let allowed_paths = self.probe.active_plan_paths()?;
        let _ = Self::path_from_resource_uri(uri, &allowed_paths)?;
        Err(VibeError::InvalidArguments(format!(
            "unknown active plan resource `{uri}`"
        )))
    }

    pub fn resource_from_path(path: &str) -> ActivePlanResource {
        ActivePlanResource {
            uri: active_plan_resource_uri(path),
            name: active_plan_resource_name(path),
            path: path.to_string(),
            mime_type: "text/markdown".to_string(),
        }
    }

    pub fn path_from_resource_uri(
        uri: &str,
        allowed_paths: &[String],
    ) -> Result<String, VibeError> {
        let _ = allowed_paths;
        Err(VibeError::InvalidArguments(format!(
            "unknown active plan resource `{uri}`"
        )))
    }
}

pub fn active_plan_resource_uri(path: &str) -> String {
    format!("{ACTIVE_PLAN_RESOURCE_URI_PREFIX}{path}")
}

pub fn active_plan_resource_name(path: &str) -> String {
    path.to_string()
}

impl<P: WorkspaceProbe> PlanValidationService<P> {
    pub fn new(probe: P) -> Self {
        Self { probe }
    }

    pub fn evaluate_active_plans(&self) -> Result<ActivePlansValidationReport, VibeError> {
        let plans = self
            .probe
            .active_plan_paths()?
            .into_iter()
            .map(|path| {
                let text = self.probe.read_text_file(&path)?;
                Ok(Self::evaluate_plan(&path, &text))
            })
            .collect::<Result<Vec<_>, VibeError>>()?;
        let ready = plans.iter().all(|plan| plan.ready);

        Ok(ActivePlansValidationReport {
            project_name: "vibe-sentinel".to_string(),
            ready,
            plans,
        })
    }

    fn evaluate_plan(path: &str, text: &str) -> PlanValidationReport {
        let document = Self::parse_plan_document(path, text);
        let checks = vec![
            Self::rule_reviewed_plan_not_pending(&document),
            Self::rule_reviewed_architecture_not_pending(&document),
            Self::rule_implementation_requires_skeletons(&document),
            Self::rule_implementation_requires_mock_tests(&document),
            Self::rule_checked_implementation_items_require_validation_notes(&document),
            Self::rule_implementation_requires_validator_pass_log(&document),
        ];
        let issues = checks
            .iter()
            .filter(|check| check.state == ValidationState::Missing)
            .map(|check| ValidationIssue {
                rule_id: check.rule_id.clone(),
                severity: check.severity.clone(),
                message: check.message.clone(),
                evidence: check.evidence.clone(),
            })
            .collect::<Vec<_>>();
        let ready = issues.is_empty();

        PlanValidationReport {
            path: path.to_string(),
            ready,
            checks,
            issues,
        }
    }

    fn parse_plan_document(path: &str, text: &str) -> ActivePlanDocument {
        let mut document = ActivePlanDocument {
            path: path.to_string(),
            title: text
                .lines()
                .find_map(|line| line.strip_prefix("# "))
                .unwrap_or("")
                .to_string(),
            reviewed_plan_status: None,
            reviewed_architecture_status: None,
            skeleton_items: Vec::new(),
            mock_test_items: Vec::new(),
            implementation_items: Vec::new(),
            validation_log_entries: Vec::new(),
        };
        let mut section = PlanSection::Other;
        let mut last_implementation_item = None;

        for (index, line) in text.lines().enumerate() {
            let line_number = index + 1;
            let trimmed = line.trim();

            if let Some(heading) = trimmed.strip_prefix("### ") {
                section = match heading.trim() {
                    "Reviewed Plan" => PlanSection::ReviewedPlan,
                    "Reviewed Architecture" => PlanSection::ReviewedArchitecture,
                    "Skeleton Checklist" => PlanSection::SkeletonChecklist,
                    "Mock Test Checklist" => PlanSection::MockTestChecklist,
                    "Implementation Checklist" => PlanSection::ImplementationChecklist,
                    "Validation Log" => PlanSection::ValidationLog,
                    _ => PlanSection::Other,
                };
                last_implementation_item = None;
                continue;
            }

            match section {
                PlanSection::ReviewedPlan => {
                    if let Some(status) =
                        Self::parse_section_status(trimmed, line_number, "Plan review status:")
                    {
                        document.reviewed_plan_status = Some(status);
                    }
                }
                PlanSection::ReviewedArchitecture => {
                    if let Some(status) = Self::parse_section_status(
                        trimmed,
                        line_number,
                        "Architecture review status:",
                    ) {
                        document.reviewed_architecture_status = Some(status);
                    }
                }
                PlanSection::SkeletonChecklist => {
                    if let Some(item) = Self::parse_checklist_item(trimmed, line_number) {
                        document.skeleton_items.push(item);
                    }
                }
                PlanSection::MockTestChecklist => {
                    if let Some(item) = Self::parse_checklist_item(trimmed, line_number) {
                        document.mock_test_items.push(item);
                    }
                }
                PlanSection::ImplementationChecklist => {
                    if let Some(item) = Self::parse_implementation_item(trimmed, line_number) {
                        document.implementation_items.push(item);
                        last_implementation_item = Some(document.implementation_items.len() - 1);
                    } else if let Some(validation_after) = Self::parse_validation_after(trimmed) {
                        if let Some(item_index) = last_implementation_item {
                            let item = &mut document.implementation_items[item_index];
                            item.validation_after_text = Some(validation_after);
                            item.validation_after_line = Some(line_number);
                        }
                    }
                }
                PlanSection::ValidationLog => {
                    if let Some(entry) = trimmed.strip_prefix("- ") {
                        document.validation_log_entries.push(ValidationLogEntry {
                            text: entry.trim().to_string(),
                            line: line_number,
                        });
                    }
                }
                PlanSection::Other => {}
            }
        }

        document
    }

    fn parse_section_status(
        trimmed_line: &str,
        line: usize,
        status_label: &str,
    ) -> Option<SectionStatus> {
        let text = Self::strip_list_marker(trimmed_line);
        text.strip_prefix(status_label)
            .map(str::trim)
            .map(|value| SectionStatus {
                value: value.to_string(),
                line,
                excerpt: trimmed_line.to_string(),
            })
    }

    fn parse_checklist_item(trimmed_line: &str, line: usize) -> Option<ChecklistItem> {
        let text = Self::strip_list_marker(trimmed_line);
        let (checked, rest) = if let Some(rest) = text.strip_prefix("[x]") {
            (true, rest)
        } else if let Some(rest) = text.strip_prefix("[X]") {
            (true, rest)
        } else if let Some(rest) = text.strip_prefix("[ ]") {
            (false, rest)
        } else {
            return None;
        };

        Some(ChecklistItem {
            text: rest.trim().to_string(),
            checked,
            line,
        })
    }

    fn parse_implementation_item(trimmed_line: &str, line: usize) -> Option<ImplementationItem> {
        Self::parse_checklist_item(trimmed_line, line).map(|item| ImplementationItem {
            text: item.text,
            checked: item.checked,
            line: item.line,
            validation_after_text: None,
            validation_after_line: None,
        })
    }

    fn parse_validation_after(trimmed_line: &str) -> Option<String> {
        let text = Self::strip_list_marker(trimmed_line);
        text.strip_prefix("Validation after")
            .and_then(|rest| rest.split_once(':'))
            .map(|(_, value)| value.trim().to_string())
    }

    fn strip_list_marker(trimmed_line: &str) -> &str {
        trimmed_line
            .strip_prefix("- ")
            .unwrap_or(trimmed_line)
            .trim()
    }

    fn section_status_ready(status: &Option<SectionStatus>) -> bool {
        status.as_ref().is_some_and(|status| {
            let value = status.value.trim();
            !value.is_empty() && !value.eq_ignore_ascii_case("pending")
        })
    }

    fn rule_reviewed_plan_not_pending(document: &ActivePlanDocument) -> ValidationCheck {
        if Self::section_status_ready(&document.reviewed_plan_status) {
            return Self::ready_check(
                "reviewed_plan_not_pending",
                ValidationSeverity::Error,
                "Reviewed Plan is recorded.".to_string(),
                document
                    .reviewed_plan_status
                    .as_ref()
                    .map(|status| Self::status_evidence("Reviewed Plan", status)),
            );
        }

        Self::missing_check(
            "reviewed_plan_not_pending",
            ValidationSeverity::Error,
            "Reviewed Plan is pending or missing.".to_string(),
            Some(Self::status_or_missing_evidence(
                "Reviewed Plan",
                document.reviewed_plan_status.as_ref(),
            )),
        )
    }

    fn rule_reviewed_architecture_not_pending(document: &ActivePlanDocument) -> ValidationCheck {
        if Self::section_status_ready(&document.reviewed_architecture_status) {
            return Self::ready_check(
                "reviewed_architecture_not_pending",
                ValidationSeverity::Error,
                "Reviewed Architecture is recorded.".to_string(),
                document
                    .reviewed_architecture_status
                    .as_ref()
                    .map(|status| Self::status_evidence("Reviewed Architecture", status)),
            );
        }

        Self::missing_check(
            "reviewed_architecture_not_pending",
            ValidationSeverity::Error,
            "Reviewed Architecture is pending or missing.".to_string(),
            Some(Self::status_or_missing_evidence(
                "Reviewed Architecture",
                document.reviewed_architecture_status.as_ref(),
            )),
        )
    }

    fn rule_implementation_requires_skeletons(document: &ActivePlanDocument) -> ValidationCheck {
        if !Self::has_checked_implementation(document)
            || document.skeleton_items.iter().any(|item| item.checked)
        {
            return Self::ready_check(
                "implementation_requires_skeletons",
                ValidationSeverity::Error,
                "Checked implementation items have skeleton evidence.".to_string(),
                document
                    .skeleton_items
                    .iter()
                    .find(|item| item.checked)
                    .map(|item| Self::checklist_evidence("Skeleton Checklist", item)),
            );
        }

        Self::missing_check(
            "implementation_requires_skeletons",
            ValidationSeverity::Error,
            "Checked implementation items require at least one checked skeleton item.".to_string(),
            Self::first_checked_implementation_evidence(document),
        )
    }

    fn rule_implementation_requires_mock_tests(document: &ActivePlanDocument) -> ValidationCheck {
        if !Self::has_checked_implementation(document)
            || document.mock_test_items.iter().any(|item| item.checked)
        {
            return Self::ready_check(
                "implementation_requires_mock_tests",
                ValidationSeverity::Error,
                "Checked implementation items have mock-test evidence.".to_string(),
                document
                    .mock_test_items
                    .iter()
                    .find(|item| item.checked)
                    .map(|item| Self::checklist_evidence("Mock Test Checklist", item)),
            );
        }

        Self::missing_check(
            "implementation_requires_mock_tests",
            ValidationSeverity::Error,
            "Checked implementation items require at least one checked mock-test item.".to_string(),
            Self::first_checked_implementation_evidence(document),
        )
    }

    fn rule_checked_implementation_items_require_validation_notes(
        document: &ActivePlanDocument,
    ) -> ValidationCheck {
        if let Some(item) = document
            .implementation_items
            .iter()
            .filter(|item| item.checked)
            .find(|item| !Self::validation_after_ready(item))
        {
            return Self::missing_check(
                "checked_implementation_items_require_validation_notes",
                ValidationSeverity::Error,
                "Checked implementation items require non-pending validation notes.".to_string(),
                Some(Self::implementation_evidence(item)),
            );
        }

        Self::ready_check(
            "checked_implementation_items_require_validation_notes",
            ValidationSeverity::Error,
            "Checked implementation items include validation notes.".to_string(),
            Self::first_checked_implementation_evidence(document),
        )
    }

    fn rule_implementation_requires_validator_pass_log(
        document: &ActivePlanDocument,
    ) -> ValidationCheck {
        if !Self::has_checked_implementation(document)
            || document.validation_log_entries.iter().any(|entry| {
                entry
                    .text
                    .contains("python3 scripts/validate_tdd_workflow.py")
                    && entry.text.to_ascii_lowercase().contains("passed")
            })
        {
            return Self::ready_check(
                "implementation_requires_validator_pass_log",
                ValidationSeverity::Error,
                "Validation log includes a TDD validator pass when implementation is checked."
                    .to_string(),
                document
                    .validation_log_entries
                    .iter()
                    .find(|entry| {
                        entry
                            .text
                            .contains("python3 scripts/validate_tdd_workflow.py")
                            && entry.text.to_ascii_lowercase().contains("passed")
                    })
                    .map(|entry| ValidationEvidence {
                        section: "Validation Log".to_string(),
                        line: Some(entry.line),
                        excerpt: entry.text.clone(),
                    }),
            );
        }

        Self::missing_check(
            "implementation_requires_validator_pass_log",
            ValidationSeverity::Error,
            "Checked implementation items require a passed TDD validator entry in the Validation Log."
                .to_string(),
            Self::first_checked_implementation_evidence(document),
        )
    }

    fn has_checked_implementation(document: &ActivePlanDocument) -> bool {
        document
            .implementation_items
            .iter()
            .any(|item| item.checked)
    }

    fn validation_after_ready(item: &ImplementationItem) -> bool {
        item.validation_after_text.as_ref().is_some_and(|text| {
            let text = text.trim();
            !text.is_empty() && !text.eq_ignore_ascii_case("pending")
        })
    }

    fn status_evidence(section: &str, status: &SectionStatus) -> ValidationEvidence {
        ValidationEvidence {
            section: section.to_string(),
            line: Some(status.line),
            excerpt: status.excerpt.clone(),
        }
    }

    fn status_or_missing_evidence(
        section: &str,
        status: Option<&SectionStatus>,
    ) -> ValidationEvidence {
        status.map_or_else(
            || ValidationEvidence {
                section: section.to_string(),
                line: None,
                excerpt: "status line missing".to_string(),
            },
            |status| Self::status_evidence(section, status),
        )
    }

    fn checklist_evidence(section: &str, item: &ChecklistItem) -> ValidationEvidence {
        ValidationEvidence {
            section: section.to_string(),
            line: Some(item.line),
            excerpt: item.text.clone(),
        }
    }

    fn implementation_evidence(item: &ImplementationItem) -> ValidationEvidence {
        ValidationEvidence {
            section: "Implementation Checklist".to_string(),
            line: item.validation_after_line.or(Some(item.line)),
            excerpt: item
                .validation_after_text
                .clone()
                .unwrap_or_else(|| item.text.clone()),
        }
    }

    fn first_checked_implementation_evidence(
        document: &ActivePlanDocument,
    ) -> Option<ValidationEvidence> {
        document
            .implementation_items
            .iter()
            .find(|item| item.checked)
            .map(Self::implementation_evidence)
    }

    fn missing_check(
        rule_id: &str,
        severity: ValidationSeverity,
        message: String,
        evidence: Option<ValidationEvidence>,
    ) -> ValidationCheck {
        ValidationCheck {
            rule_id: rule_id.to_string(),
            state: ValidationState::Missing,
            severity,
            message,
            evidence,
        }
    }

    fn ready_check(
        rule_id: &str,
        severity: ValidationSeverity,
        message: String,
        evidence: Option<ValidationEvidence>,
    ) -> ValidationCheck {
        ValidationCheck {
            rule_id: rule_id.to_string(),
            state: ValidationState::Ready,
            severity,
            message,
            evidence,
        }
    }
}

impl<P: WorkspaceProbe> TddGateService<P> {
    pub fn new(probe: P) -> Self {
        Self { probe }
    }

    pub fn evaluate(&self, next_action: TddGateAction) -> Result<TddGateReport, VibeError> {
        let report = PlanValidationService::new(&self.probe).evaluate_active_plans()?;

        Ok(Self::report_from_validation(report, next_action))
    }

    fn report_from_validation(
        report: ActivePlansValidationReport,
        next_action: TddGateAction,
    ) -> TddGateReport {
        let current_phase = Self::current_phase(&report);
        let blocking_issues = Self::blocking_issues_for_action(&report, next_action);

        TddGateReport {
            project_name: report.project_name,
            allowed: blocking_issues.is_empty(),
            current_phase,
            blocking_issues,
            warnings: Vec::new(),
            next_allowed_actions: Self::next_allowed_actions(current_phase),
        }
    }

    fn current_phase(report: &ActivePlansValidationReport) -> TddWorkflowPhase {
        if report.plans.is_empty() {
            return TddWorkflowPhase::Idle;
        }

        if Self::has_missing_rule(report, "reviewed_plan_not_pending") {
            return TddWorkflowPhase::PlanCreated;
        }

        if Self::has_missing_rule(report, "reviewed_architecture_not_pending") {
            return TddWorkflowPhase::PlanReviewed;
        }

        if Self::implementation_evidence_present(report) {
            return if report.ready {
                TddWorkflowPhase::CompleteReady
            } else {
                TddWorkflowPhase::ImplementationUnderway
            };
        }

        if report.ready {
            TddWorkflowPhase::ImplementationReady
        } else {
            TddWorkflowPhase::ArchitectureReviewed
        }
    }

    fn next_allowed_actions(phase: TddWorkflowPhase) -> Vec<TddGateAction> {
        match phase {
            TddWorkflowPhase::Idle => Vec::new(),
            TddWorkflowPhase::PlanCreated => vec![TddGateAction::StartArchitecture],
            TddWorkflowPhase::PlanReviewed => vec![TddGateAction::StartSkeletons],
            TddWorkflowPhase::ArchitectureReviewed => vec![TddGateAction::StartMockTests],
            TddWorkflowPhase::ImplementationReady => vec![TddGateAction::StartImplementation],
            TddWorkflowPhase::ImplementationUnderway | TddWorkflowPhase::CompleteReady => {
                vec![TddGateAction::CompletePlan]
            }
        }
    }

    fn blocking_issues_for_action(
        report: &ActivePlansValidationReport,
        next_action: TddGateAction,
    ) -> Vec<ValidationIssue> {
        if report.plans.is_empty() {
            return vec![Self::issue(
                "active_plan_required",
                "An active execution plan is required before feature workflow actions.",
            )];
        }

        match next_action {
            TddGateAction::StartArchitecture => Vec::new(),
            TddGateAction::StartSkeletons => {
                Self::issues_for_rules(report, &["reviewed_plan_not_pending"])
            }
            TddGateAction::StartMockTests => Self::issues_for_rules(
                report,
                &[
                    "reviewed_plan_not_pending",
                    "reviewed_architecture_not_pending",
                ],
            ),
            TddGateAction::StartImplementation => report
                .plans
                .iter()
                .flat_map(|plan| plan.issues.clone())
                .collect(),
            TddGateAction::CompletePlan => {
                let mut issues = report
                    .plans
                    .iter()
                    .flat_map(|plan| plan.issues.clone())
                    .collect::<Vec<_>>();
                if !Self::implementation_evidence_present(report) {
                    issues.push(Self::issue(
                        "implementation_evidence_required",
                        "Completing a plan requires checked implementation work with validation evidence.",
                    ));
                }
                issues
            }
        }
    }

    fn implementation_evidence_present(report: &ActivePlansValidationReport) -> bool {
        report.plans.iter().any(|plan| {
            plan.checks.iter().any(|check| {
                check.rule_id == "checked_implementation_items_require_validation_notes"
                    && check.evidence.is_some()
            }) || plan.issues.iter().any(|issue| {
                matches!(
                    issue.rule_id.as_str(),
                    "checked_implementation_items_require_validation_notes"
                        | "implementation_requires_skeletons"
                        | "implementation_requires_mock_tests"
                ) && issue.evidence.is_some()
            })
        })
    }

    fn has_missing_rule(report: &ActivePlansValidationReport, rule_id: &str) -> bool {
        report.plans.iter().any(|plan| {
            plan.checks
                .iter()
                .any(|check| check.rule_id == rule_id && check.state == ValidationState::Missing)
        })
    }

    fn issues_for_rules(
        report: &ActivePlansValidationReport,
        rule_ids: &[&str],
    ) -> Vec<ValidationIssue> {
        report
            .plans
            .iter()
            .flat_map(|plan| plan.issues.iter())
            .filter(|issue| rule_ids.iter().any(|rule_id| *rule_id == issue.rule_id))
            .cloned()
            .collect()
    }

    fn issue(rule_id: &str, message: &str) -> ValidationIssue {
        ValidationIssue {
            rule_id: rule_id.to_string(),
            severity: ValidationSeverity::Error,
            message: message.to_string(),
            evidence: None,
        }
    }
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
    use super::{PlanValidationService, StatusService, TddGateService};
    use crate::adapters::test_support::FakeWorkspaceProbe;
    use crate::domain::{ReadinessState, TddGateAction, TddWorkflowPhase, ValidationState};

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

    #[test]
    fn active_plan_validation_reports_no_active_plans() {
        let service = PlanValidationService::new(FakeWorkspaceProbe::new());

        let report = service.evaluate_active_plans().expect("validation report");

        assert_eq!(report.project_name, "vibe-sentinel");
        assert!(report.ready);
        assert!(report.plans.is_empty());
    }

    #[test]
    fn active_plan_validation_reports_ready_plan() {
        let service = PlanValidationService::new(
            FakeWorkspaceProbe::new()
                .with_active_plan_file("docs/exec-plans/active/ready.md", ready_plan_text()),
        );

        let report = service.evaluate_active_plans().expect("validation report");

        assert!(report.ready);
        assert_eq!(report.plans.len(), 1);
        assert_eq!(report.plans[0].path, "docs/exec-plans/active/ready.md");
        assert_eq!(report.plans[0].checks.len(), 6);
        assert!(report.plans[0].issues.is_empty());
        assert!(report.plans[0]
            .checks
            .iter()
            .all(|check| check.state == ValidationState::Ready));
    }

    #[test]
    fn active_plan_validation_reports_pending_review_statuses() {
        let service = PlanValidationService::new(FakeWorkspaceProbe::new().with_active_plan_file(
            "docs/exec-plans/active/pending.md",
            &ready_plan_text().replace(
                "Plan review status: reviewed",
                "Plan review status: pending",
            ),
        ));

        let report = service.evaluate_active_plans().expect("validation report");

        assert!(!report.ready);
        assert!(report.plans[0].issues.iter().any(|issue| {
            issue.rule_id == "reviewed_plan_not_pending" && issue.message.contains("Reviewed Plan")
        }));
    }

    #[test]
    fn active_plan_validation_requires_skeletons_before_implementation() {
        let text = ready_plan_text().replace(
            "- [x] `domain` skeleton added.",
            "- [ ] `domain` skeleton added.",
        );
        let service = PlanValidationService::new(
            FakeWorkspaceProbe::new()
                .with_active_plan_file("docs/exec-plans/active/no-skeleton.md", &text),
        );

        let report = service.evaluate_active_plans().expect("validation report");

        assert!(!report.ready);
        assert!(report.plans[0]
            .issues
            .iter()
            .any(|issue| issue.rule_id == "implementation_requires_skeletons"));
    }

    #[test]
    fn active_plan_validation_requires_mock_tests_before_implementation() {
        let text = ready_plan_text().replace(
            "- [x] core test covers skeleton behavior.",
            "- [ ] core test covers skeleton behavior.",
        );
        let service = PlanValidationService::new(
            FakeWorkspaceProbe::new()
                .with_active_plan_file("docs/exec-plans/active/no-mocks.md", &text),
        );

        let report = service.evaluate_active_plans().expect("validation report");

        assert!(!report.ready);
        assert!(report.plans[0]
            .issues
            .iter()
            .any(|issue| issue.rule_id == "implementation_requires_mock_tests"));
    }

    #[test]
    fn active_plan_validation_requires_validation_notes_for_checked_implementation_items() {
        let text = ready_plan_text().replace(
            "- Validation after this unit: `cargo test active_plan_validation_reports_ready_plan` passed.",
            "- Validation after this unit: pending",
        );
        let service = PlanValidationService::new(
            FakeWorkspaceProbe::new()
                .with_active_plan_file("docs/exec-plans/active/no-validation-note.md", &text),
        );

        let report = service.evaluate_active_plans().expect("validation report");

        assert!(!report.ready);
        assert!(report.plans[0].issues.iter().any(|issue| {
            issue.rule_id == "checked_implementation_items_require_validation_notes"
        }));
    }

    #[test]
    fn active_plan_validation_requires_tdd_validator_pass_log() {
        let text = ready_plan_text().replace(
            "- 2026-05-07: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/example.md` -> passed.",
            "- 2026-05-07: `cargo test --all` -> passed.",
        );
        let service = PlanValidationService::new(
            FakeWorkspaceProbe::new()
                .with_active_plan_file("docs/exec-plans/active/no-validator-log.md", &text),
        );

        let report = service.evaluate_active_plans().expect("validation report");

        assert!(!report.ready);
        assert!(report.plans[0]
            .issues
            .iter()
            .any(|issue| issue.rule_id == "implementation_requires_validator_pass_log"));
    }

    #[test]
    fn tdd_gate_blocks_idle_workspace_actions() {
        let service = TddGateService::new(FakeWorkspaceProbe::new());

        let report = service
            .evaluate(TddGateAction::StartArchitecture)
            .expect("gate report");

        assert!(!report.allowed);
        assert_eq!(report.current_phase, TddWorkflowPhase::Idle);
        assert!(report.next_allowed_actions.is_empty());
        assert!(report
            .blocking_issues
            .iter()
            .any(|issue| issue.rule_id == "active_plan_required"));
    }

    #[test]
    fn tdd_gate_allows_architecture_when_plan_exists() {
        let service = TddGateService::new(
            FakeWorkspaceProbe::new()
                .with_active_plan_file("docs/exec-plans/active/plan.md", plan_created_text()),
        );

        let report = service
            .evaluate(TddGateAction::StartArchitecture)
            .expect("gate report");

        assert!(report.allowed);
        assert_eq!(report.current_phase, TddWorkflowPhase::PlanCreated);
        assert_eq!(
            report.next_allowed_actions,
            vec![TddGateAction::StartArchitecture]
        );
    }

    #[test]
    fn tdd_gate_blocks_implementation_until_validation_ready() {
        let text = ready_plan_text().replace(
            "- [x] `domain` skeleton added.",
            "- [ ] `domain` skeleton added.",
        );
        let service = TddGateService::new(
            FakeWorkspaceProbe::new()
                .with_active_plan_file("docs/exec-plans/active/not-ready.md", &text),
        );

        let report = service
            .evaluate(TddGateAction::StartImplementation)
            .expect("gate report");

        assert!(!report.allowed);
        assert_eq!(
            report.current_phase,
            TddWorkflowPhase::ImplementationUnderway
        );
        assert!(report
            .blocking_issues
            .iter()
            .any(|issue| issue.rule_id == "implementation_requires_skeletons"));
    }

    #[test]
    fn tdd_gate_allows_implementation_when_active_plan_validation_is_ready() {
        let service = TddGateService::new(FakeWorkspaceProbe::new().with_active_plan_file(
            "docs/exec-plans/active/implementation-ready.md",
            implementation_ready_plan_text(),
        ));

        let report = service
            .evaluate(TddGateAction::StartImplementation)
            .expect("gate report");

        assert!(report.allowed);
        assert_eq!(report.current_phase, TddWorkflowPhase::ImplementationReady);
        assert_eq!(
            report.next_allowed_actions,
            vec![TddGateAction::StartImplementation]
        );
    }

    fn ready_plan_text() -> &'static str {
        r#"# Execution Plan: Example

## TDD artifacts

### Reviewed Plan

- Plan review status: reviewed
- Refinements made: Ready for architecture.

### Reviewed Architecture

- Architecture review status: reviewed
- Refinements made: Ready for skeletons.

### Skeleton Checklist

- [x] `domain` skeleton added.

### Mock Test Checklist

- [x] core test covers skeleton behavior.

### Implementation Checklist

- [x] Fill `domain` behavior.
- Validation after this unit: `cargo test active_plan_validation_reports_ready_plan` passed.

### Validation Log

- 2026-05-07: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/example.md` -> passed.

### Review Notes

- Diff review: pending.
"#
    }

    fn plan_created_text() -> &'static str {
        r#"# Execution Plan: Created

## TDD artifacts

### Reviewed Plan

- Plan review status: pending

### Reviewed Architecture

- Architecture review status: pending

### Skeleton Checklist

- [ ] `domain` skeleton added.

### Mock Test Checklist

- [ ] core test covers skeleton behavior.

### Implementation Checklist

- [ ] Fill `domain` behavior.
- Validation after this unit: pending

### Validation Log

- pending
"#
    }

    fn implementation_ready_plan_text() -> &'static str {
        r#"# Execution Plan: Implementation Ready

## TDD artifacts

### Reviewed Plan

- Plan review status: reviewed

### Reviewed Architecture

- Architecture review status: reviewed

### Skeleton Checklist

- [x] `domain` skeleton added.

### Mock Test Checklist

- [x] core test covers skeleton behavior.

### Implementation Checklist

- [ ] Fill `domain` behavior.
- Validation after this unit: pending

### Validation Log

- 2026-05-07: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/example.md` -> passed.
"#
    }
}
