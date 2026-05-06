use crate::core::StatusService;
use crate::domain::{ReadinessState, StatusReport, VibeError};
use crate::ports::WorkspaceProbe;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliArgs {
    pub command: CliCommand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliCommand {
    Status,
}

pub fn parse_args<I, S>(args: I) -> Result<CliArgs, VibeError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut arguments: Vec<String> = args.into_iter().map(Into::into).collect();
    if matches!(arguments.first(), Some(binary_name) if binary_name.ends_with("vibe-sentinel")) {
        arguments.remove(0);
    }

    match arguments.as_slice() {
        [command] if command == "status" => Ok(CliArgs {
            command: CliCommand::Status,
        }),
        [] => Err(VibeError::InvalidArguments(
            "missing command: expected `status`".to_string(),
        )),
        [command, ..] => Err(VibeError::InvalidArguments(format!(
            "unknown command `{command}`: expected `status`"
        ))),
    }
}

pub fn execute_with_probe<P: WorkspaceProbe>(
    args: CliArgs,
    probe: P,
) -> Result<StatusReport, VibeError> {
    let _args = args;
    StatusService::new(probe).evaluate()
}

pub fn format_status(report: &StatusReport) -> String {
    let mut output = format!("{} status\n", report.project_name);
    for check in &report.checks {
        let state = match check.state {
            ReadinessState::Ready => "ready",
            ReadinessState::Missing => "missing",
        };
        output.push_str(&format!("- {}: {} - {}\n", check.name, state, check.detail));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::{execute_with_probe, format_status, parse_args, CliCommand};
    use crate::adapters::test_support::FakeWorkspaceProbe;
    use crate::domain::VibeError;

    #[test]
    fn parse_args_skeleton_returns_status_command() {
        let args = parse_args(["vibe-sentinel", "status"]).expect("parsed args");

        assert_eq!(args.command, CliCommand::Status);
    }

    #[test]
    fn parse_args_rejects_unknown_command() {
        let error = parse_args(["vibe-sentinel", "watch"]).expect_err("parse error");

        assert_eq!(
            error,
            VibeError::InvalidArguments("unknown command `watch`: expected `status`".to_string())
        );
    }

    #[test]
    fn execute_with_probe_returns_status_report() {
        let args = parse_args(["vibe-sentinel", "status"]).expect("parsed args");

        let report = execute_with_probe(args, FakeWorkspaceProbe::new()).expect("status report");

        assert_eq!(report.project_name, "vibe-sentinel");
    }

    #[test]
    fn format_status_is_deterministic() {
        let args = parse_args(["vibe-sentinel", "status"]).expect("parsed args");
        let report = execute_with_probe(
            args,
            FakeWorkspaceProbe::new()
                .with_path("AGENTS.md")
                .with_path("docs/harness/scope.md")
                .with_path("docs/harness/operating-model.md")
                .with_path("Cargo.toml")
                .with_active_plan(true),
        )
        .expect("status report");

        assert_eq!(
            format_status(&report),
            "vibe-sentinel status\n- harness docs: ready - required harness docs present\n- active plan: ready - active execution plan present\n- rust workspace: ready - Cargo workspace present\n"
        );
    }
}
