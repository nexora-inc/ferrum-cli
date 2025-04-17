use crate::{core::Result, types::utils::file_util::FerrumConfigProfile, utils::FileUtilProvider};

pub trait LoadEnvironmentVariablesActionProvider {
  fn execute(&self, profile: &FerrumConfigProfile) -> Result<Vec<String>>;
  }

pub struct LoadEnvironmentVariablesAction<'a, F: FileUtilProvider> {
  file_util: &'a F,
}

impl<'a, F: FileUtilProvider> LoadEnvironmentVariablesAction<'a, F> {
  pub fn new(file_util: &'a F) -> Self { Self { file_util } }
}

impl<'a, F: FileUtilProvider> LoadEnvironmentVariablesActionProvider
  for LoadEnvironmentVariablesAction<'a, F> {
    fn execute(&self, profile: &FerrumConfigProfile) -> Result<Vec<String>> {
      println!("🗂️ Reading environment variables from {}...", &profile.env_path);
      let env_variable_lines = self.file_util.read_lines(&profile.env_path)?;
      println!("✅ Environment variables loaded");

      Ok(env_variable_lines)
    }
  }
