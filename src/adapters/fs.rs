use std::path::PathBuf;

use crate::domain::VibeError;
use crate::ports::WorkspaceProbe;

pub struct FsWorkspaceProbe {
    root: PathBuf,
}

impl FsWorkspaceProbe {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}

impl WorkspaceProbe for FsWorkspaceProbe {
    fn exists(&self, relative_path: &str) -> Result<bool, VibeError> {
        Ok(self.root.join(relative_path).exists())
    }

    fn has_any_active_plan(&self) -> Result<bool, VibeError> {
        Ok(!self.active_plan_paths()?.is_empty())
    }

    fn active_plan_paths(&self) -> Result<Vec<String>, VibeError> {
        let active_dir = self.root.join("docs/exec-plans/active");
        if !active_dir.exists() {
            return Ok(Vec::new());
        }

        let entries = std::fs::read_dir(&active_dir).map_err(|error| {
            VibeError::WorkspaceUnreadable(format!(
                "could not read active plan directory `{}`: {error}",
                active_dir.display()
            ))
        })?;

        let mut paths = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|error| {
                VibeError::WorkspaceUnreadable(format!(
                    "could not inspect active plan directory `{}`: {error}",
                    active_dir.display()
                ))
            })?;
            let path = entry.path();
            let is_markdown = path.extension().is_some_and(|extension| extension == "md");
            let is_readme = path
                .file_name()
                .and_then(|file_name| file_name.to_str())
                .is_some_and(|file_name| file_name.eq_ignore_ascii_case("README.md"));
            if path.is_file() && is_markdown && !is_readme {
                if let Some(file_name) = path.file_name().and_then(|file_name| file_name.to_str()) {
                    paths.push(format!("docs/exec-plans/active/{file_name}"));
                }
            }
        }

        paths.sort();
        Ok(paths)
    }

    fn read_text_file(&self, relative_path: &str) -> Result<String, VibeError> {
        std::fs::read_to_string(self.root.join(relative_path)).map_err(|error| {
            VibeError::WorkspaceUnreadable(format!(
                "could not read workspace file `{relative_path}`: {error}"
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::FsWorkspaceProbe;
    use crate::ports::WorkspaceProbe;

    #[test]
    fn fs_workspace_probe_detects_active_plan_files() {
        let root = unique_test_root();
        let active_dir = root.join("docs/exec-plans/active");
        fs::create_dir_all(&active_dir).expect("active dir");
        fs::write(active_dir.join("README.md"), "# Active plans\n").expect("readme");
        fs::write(active_dir.join("product-bootstrap.md"), "# Plan\n").expect("plan");

        let probe = FsWorkspaceProbe::new(root.clone());

        assert!(probe.has_any_active_plan().expect("active plan check"));

        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn fs_workspace_probe_lists_active_plan_files_in_deterministic_order() {
        let root = unique_test_root();
        let active_dir = root.join("docs/exec-plans/active");
        fs::create_dir_all(&active_dir).expect("active dir");
        fs::write(active_dir.join("README.md"), "# Active plans\n").expect("readme");
        fs::write(active_dir.join("b-plan.md"), "# B\n").expect("b plan");
        fs::write(active_dir.join("a-plan.md"), "# A\n").expect("a plan");
        fs::write(active_dir.join("notes.txt"), "ignore\n").expect("notes");

        let probe = FsWorkspaceProbe::new(root.clone());

        assert_eq!(
            probe.active_plan_paths().expect("active plan paths"),
            vec![
                "docs/exec-plans/active/a-plan.md".to_string(),
                "docs/exec-plans/active/b-plan.md".to_string()
            ]
        );

        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn fs_workspace_probe_reads_text_files_under_workspace_root() {
        let root = unique_test_root();
        fs::create_dir_all(root.join("docs/exec-plans/active")).expect("active dir");
        fs::write(root.join("docs/exec-plans/active/plan.md"), "# Plan\n").expect("plan");

        let probe = FsWorkspaceProbe::new(root.clone());

        assert_eq!(
            probe
                .read_text_file("docs/exec-plans/active/plan.md")
                .expect("plan text"),
            "# Plan\n"
        );

        fs::remove_dir_all(root).expect("cleanup");
    }

    fn unique_test_root() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let base = std::env::var_os("TMPDIR")
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir);
        base.join(format!("vibe-sentinel-test-{nanos}"))
    }
}
