use crate::domain::VibeError;
use crate::ports::WorkspaceProbe;

#[derive(Debug, Default, Clone)]
pub struct FakeWorkspaceProbe {
    existing_paths: Vec<String>,
    has_active_plan: bool,
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
}
