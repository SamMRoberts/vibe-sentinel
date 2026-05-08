use crate::domain::VibeError;

pub trait WorkspaceProbe {
    fn exists(&self, relative_path: &str) -> Result<bool, VibeError>;

    fn has_any_active_plan(&self) -> Result<bool, VibeError>;

    fn active_plan_paths(&self) -> Result<Vec<String>, VibeError>;

    fn read_text_file(&self, relative_path: &str) -> Result<String, VibeError>;
}

impl<T: WorkspaceProbe + ?Sized> WorkspaceProbe for &T {
    fn exists(&self, relative_path: &str) -> Result<bool, VibeError> {
        (*self).exists(relative_path)
    }

    fn has_any_active_plan(&self) -> Result<bool, VibeError> {
        (*self).has_any_active_plan()
    }

    fn active_plan_paths(&self) -> Result<Vec<String>, VibeError> {
        (*self).active_plan_paths()
    }

    fn read_text_file(&self, relative_path: &str) -> Result<String, VibeError> {
        (*self).read_text_file(relative_path)
    }
}
