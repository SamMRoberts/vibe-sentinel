use crate::domain::VibeError;

pub trait WorkspaceProbe {
    fn exists(&self, relative_path: &str) -> Result<bool, VibeError>;

    fn has_any_active_plan(&self) -> Result<bool, VibeError>;
}
