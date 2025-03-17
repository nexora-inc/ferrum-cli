use std::process::Command;

use crate::core::AppError;

pub trait IServerlessDeploy {
  fn execute(&self) -> Result<(), AppError>;
}

pub struct ServerlessDeploy;

impl ServerlessDeploy {
  pub fn new() -> Self {
    Self {}
  }
}

impl IServerlessDeploy for ServerlessDeploy {
  fn execute(&self) -> Result<(), AppError> {
    println!("Executing serverless deploy command...");
    let command_output = Command::new("serverless")
      .arg("deploy")
      .output()?;

    if !command_output.status.success() {
      let std_error = String::from_utf8_lossy(&command_output.stderr);
      return Err(AppError::CommandError(std_error.to_string()))
    }

    println!("Project deployed successfully");
    Ok(())
  }
}
