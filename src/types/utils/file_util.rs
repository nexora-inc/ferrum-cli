use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FerrumConfigAws {
  pub region: String,
  pub iam_role: String,
}

#[derive(Debug, Deserialize)]
pub struct FerrumConfigProfile {
  pub env_path: String,
}

#[derive(Debug, Deserialize)]
pub struct FerrumConfig {
  pub aws: FerrumConfigAws,
  pub profiles: HashMap<String, FerrumConfigProfile>
}
