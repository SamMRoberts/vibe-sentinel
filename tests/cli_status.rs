use std::process::Command;

#[test]
fn status_command_prints_workspace_status() {
    let output = Command::new(env!("CARGO_BIN_EXE_vibe-sentinel"))
        .arg("status")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
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
