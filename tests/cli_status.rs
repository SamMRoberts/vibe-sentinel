use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn status_command_prints_workspace_status() {
    let workspace = TestWorkspace::new();
    let output = Command::new(env!("CARGO_BIN_EXE_vibe-sentinel"))
        .arg("status")
        .current_dir(workspace.root())
        .output()
        .expect("status command output");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(
        stdout,
        "vibe-sentinel status\n- harness docs: ready - required harness docs present\n- active plan: ready - active execution plan present\n- rust workspace: ready - Cargo workspace present\n"
    );
}

#[test]
fn status_json_command_prints_workspace_status_json() {
    let workspace = TestWorkspace::new();
    let output = Command::new(env!("CARGO_BIN_EXE_vibe-sentinel"))
        .args(["status", "--json"])
        .current_dir(workspace.root())
        .output()
        .expect("status command output");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(
        stdout,
        "{\"project_name\":\"vibe-sentinel\",\"ready\":true,\"checks\":[{\"name\":\"harness docs\",\"state\":\"ready\",\"detail\":\"required harness docs present\"},{\"name\":\"active plan\",\"state\":\"ready\",\"detail\":\"active execution plan present\"},{\"name\":\"rust workspace\",\"state\":\"ready\",\"detail\":\"Cargo workspace present\"}]}\n"
    );
}

struct TestWorkspace {
    root: std::path::PathBuf,
}

impl TestWorkspace {
    fn new() -> Self {
        let root = unique_test_root();
        std::fs::create_dir_all(root.join("docs/harness")).expect("harness docs dir");
        std::fs::create_dir_all(root.join("docs/exec-plans/active")).expect("active plan dir");
        std::fs::write(root.join("AGENTS.md"), "# AGENTS\n").expect("agents doc");
        std::fs::write(root.join("docs/harness/scope.md"), "# Scope\n").expect("scope doc");
        std::fs::write(
            root.join("docs/harness/operating-model.md"),
            "# Operating Model\n",
        )
        .expect("operating model doc");
        std::fs::write(root.join("docs/exec-plans/active/README.md"), "# Active\n")
            .expect("active readme");
        std::fs::write(root.join("docs/exec-plans/active/test-plan.md"), "# Plan\n")
            .expect("active plan");
        std::fs::write(root.join("Cargo.toml"), "[package]\nname = \"fixture\"\n")
            .expect("cargo manifest");
        Self { root }
    }

    fn root(&self) -> &std::path::Path {
        &self.root
    }
}

impl Drop for TestWorkspace {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

fn unique_test_root() -> std::path::PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    let base = std::env::var_os("TMPDIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    base.join(format!("vibe-sentinel-cli-test-{nanos}"))
}
