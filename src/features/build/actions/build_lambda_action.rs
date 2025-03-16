use std::process::Command;

use crate::core::AppError;

pub trait IBuildLambda {
  fn execute(&self) -> Result<(), AppError>;
}

pub struct BuildLambda;

impl BuildLambda {
  pub fn new() -> Self { Self {} }
}

impl IBuildLambda for BuildLambda {
  fn execute(&self) -> Result<(), AppError> {
    let command_output = Command::new("cargo")
      .arg("lambda build --release --target x86_64-unknown-linux-gnu")
      .output()?;

    if !command_output.status.success() {
      let std_error = String::from_utf8_lossy(&command_output.stderr);
      return Err(AppError::CommandError(std_error.to_string()))
    }

    println!("Lambda project built successfully");
    Ok(())
  }
}
