use crate::core::StatusService;
use crate::domain::{ReadinessState, StatusReport, VibeError};
use crate::ports::WorkspaceProbe;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliArgs {
    pub command: CliCommand,
    pub output_format: OutputFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CliCommand {
    Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
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
            output_format: OutputFormat::Text,
        }),
        [command, flag] if command == "status" && flag == "--json" => Ok(CliArgs {
            command: CliCommand::Status,
            output_format: OutputFormat::Json,
        }),
        [command, flag] if command == "status" => Err(VibeError::InvalidArguments(format!(
            "unknown flag `{flag}`: expected `--json`"
        ))),
        [command, ..] if command == "status" => Err(VibeError::InvalidArguments(
            "too many arguments for `status`: expected optional `--json`".to_string(),
        )),
        [] => Err(VibeError::InvalidArguments(
            "missing command: expected `status`".to_string(),
        )),
        [command, ..] => Err(VibeError::InvalidArguments(format!(
            "unknown command `{command}`: expected `status`"
        ))),
    }
}

pub fn render_status(args: &CliArgs, report: &StatusReport) -> Result<String, VibeError> {
    match args.output_format {
        OutputFormat::Text => Ok(format_status(report)),
        OutputFormat::Json => format_status_json(report),
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

pub fn format_status_json(report: &StatusReport) -> Result<String, VibeError> {
    #[derive(Serialize)]
    struct StatusJson<'a> {
        project_name: &'a str,
        ready: bool,
        checks: &'a [crate::domain::StatusCheck],
    }

    let output = StatusJson {
        project_name: &report.project_name,
        ready: report.is_ready(),
        checks: &report.checks,
    };

    serde_json::to_string(&output)
        .map(|mut json| {
            json.push('\n');
            json
        })
        .map_err(|error| {
            VibeError::StatusEvaluationFailed(format!("could not format status JSON: {error}"))
        })
}

#[cfg(test)]
mod tests {
    use super::{
        execute_with_probe, format_status, format_status_json, parse_args, CliCommand, OutputFormat,
    };
    use crate::adapters::test_support::FakeWorkspaceProbe;
    use crate::domain::VibeError;

    #[test]
    fn parse_args_skeleton_returns_status_command() {
        let args = parse_args(["vibe-sentinel", "status"]).expect("parsed args");

        assert_eq!(args.command, CliCommand::Status);
        assert_eq!(args.output_format, OutputFormat::Text);
    }

    #[test]
    fn parse_args_accepts_status_json() {
        let args = parse_args(["vibe-sentinel", "status", "--json"]).expect("parsed args");

        assert_eq!(args.command, CliCommand::Status);
        assert_eq!(args.output_format, OutputFormat::Json);
    }

    #[test]
    fn parse_args_rejects_unknown_status_flag() {
        let error = parse_args(["vibe-sentinel", "status", "--pretty"]).expect_err("parse error");

        assert_eq!(
            error,
            VibeError::InvalidArguments("unknown flag `--pretty`: expected `--json`".to_string())
        );
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

    #[test]
    fn format_status_json_is_deterministic() {
        let args = parse_args(["vibe-sentinel", "status", "--json"]).expect("parsed args");
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
            format_status_json(&report).expect("json output"),
            "{\"project_name\":\"vibe-sentinel\",\"ready\":true,\"checks\":[{\"name\":\"harness docs\",\"state\":\"ready\",\"detail\":\"required harness docs present\"},{\"name\":\"active plan\",\"state\":\"ready\",\"detail\":\"active execution plan present\"},{\"name\":\"rust workspace\",\"state\":\"ready\",\"detail\":\"Cargo workspace present\"}]}\n"
        );
    }
}
