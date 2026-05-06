use std::env;
use std::process::ExitCode;

use vibe_sentinel::adapters::fs::FsWorkspaceProbe;
use vibe_sentinel::cli::{execute_with_probe, format_status, parse_args};
use vibe_sentinel::domain::VibeError;

fn main() -> ExitCode {
    match run(env::args()) {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run<I, S>(args: I) -> Result<String, VibeError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let args = parse_args(args)?;
    let root = env::current_dir().map_err(|error| {
        VibeError::WorkspaceUnreadable(format!("could not read current directory: {error}"))
    })?;
    let report = execute_with_probe(args, FsWorkspaceProbe::new(root))?;
    Ok(format_status(&report))
}
