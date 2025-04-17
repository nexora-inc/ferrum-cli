use crate::{core::Result, types::utils::file_util::FerrumConfig, utils::FileUtilProvider};

pub trait LoadConfigurationActionProvider {
  fn execute(&self) -> Result<FerrumConfig>;
}

pub struct LoadConfigurationAction<'a, F: FileUtilProvider> {
  file_util: &'a F,
}

impl<'a, F: FileUtilProvider + 'a> LoadConfigurationAction<'a, F> {
  pub fn new(file_util: &'a F) -> Self {
    Self { file_util }
  }
}

impl<'a, F: FileUtilProvider> LoadConfigurationActionProvider for LoadConfigurationAction<'a, F> {
  fn execute(&self) -> Result<FerrumConfig> {
    println!("🗂️ Reading configuration from ferrum.json...");
    let ferrum_config = self.file_util.read_json("ferrum.json")?;
    println!("✅ Configuration loaded successfully");

    Ok(ferrum_config)
  }
}
