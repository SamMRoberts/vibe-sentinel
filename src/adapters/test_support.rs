use crate::domain::VibeError;
use crate::ports::WorkspaceProbe;

#[derive(Debug, Default, Clone)]
pub struct FakeWorkspaceProbe {
    existing_paths: Vec<String>,
    has_active_plan: bool,
    text_files: Vec<(String, String)>,
}

impl FakeWorkspaceProbe {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_path(mut self, relative_path: &str) -> Self {
        self.existing_paths.push(relative_path.to_string());
        self
    }

    pub fn with_active_plan(mut self, has_active_plan: bool) -> Self {
        self.has_active_plan = has_active_plan;
        self
    }

    pub fn with_text_file(mut self, relative_path: &str, contents: &str) -> Self {
        self.text_files
            .push((relative_path.to_string(), contents.to_string()));
        self
    }

    pub fn with_active_plan_file(self, relative_path: &str, contents: &str) -> Self {
        self.with_text_file(relative_path, contents)
            .with_active_plan(true)
    }
}

impl WorkspaceProbe for FakeWorkspaceProbe {
    fn exists(&self, relative_path: &str) -> Result<bool, VibeError> {
        Ok(self
            .existing_paths
            .iter()
            .any(|existing_path| existing_path == relative_path))
    }

    fn has_any_active_plan(&self) -> Result<bool, VibeError> {
        Ok(self.has_active_plan)
    }

    fn active_plan_paths(&self) -> Result<Vec<String>, VibeError> {
        let mut paths: Vec<String> = self
            .text_files
            .iter()
            .map(|(path, _)| path)
            .filter(|path| path.starts_with("docs/exec-plans/active/"))
            .filter(|path| path.ends_with(".md"))
            .filter(|path| !path.eq_ignore_ascii_case("docs/exec-plans/active/README.md"))
            .cloned()
            .collect();
        paths.sort();
        Ok(paths)
    }

    fn read_text_file(&self, relative_path: &str) -> Result<String, VibeError> {
        self.text_files
            .iter()
            .find(|(path, _)| path == relative_path)
            .map(|(_, contents)| contents.clone())
            .ok_or_else(|| {
                VibeError::WorkspaceUnreadable(format!(
                    "could not read workspace file `{relative_path}`"
                ))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::FakeWorkspaceProbe;
    use crate::ports::WorkspaceProbe;

    #[test]
    fn fake_workspace_probe_reports_configured_paths() {
        let probe = FakeWorkspaceProbe::new()
            .with_path("AGENTS.md")
            .with_active_plan(true);

        assert!(probe.exists("AGENTS.md").expect("path check"));
        assert!(!probe.exists("missing.md").expect("path check"));
        assert!(probe.has_any_active_plan().expect("active plan check"));
    }

    #[test]
    fn fake_workspace_probe_reports_configured_plan_files() {
        let probe = FakeWorkspaceProbe::new()
            .with_active_plan_file("docs/exec-plans/active/b.md", "# B\n")
            .with_active_plan_file("docs/exec-plans/active/a.md", "# A\n")
            .with_text_file("docs/exec-plans/active/README.md", "# Active\n");

        assert_eq!(
            probe.active_plan_paths().expect("active plan paths"),
            vec![
                "docs/exec-plans/active/a.md".to_string(),
                "docs/exec-plans/active/b.md".to_string()
            ]
        );
        assert_eq!(
            probe
                .read_text_file("docs/exec-plans/active/a.md")
                .expect("plan text"),
            "# A\n"
        );
    }
}
