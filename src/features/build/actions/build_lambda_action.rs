use std::process::Command;

use crate::core::AppError;

pub trait IBuildLambda {
  fn execute(&self) -> Result<(), AppError>;
}

pub struct BuildLambda;

impl BuildLambda {
  pub fn new() -> Self {
    Self {}
  }
}

impl IBuildLambda for BuildLambda {
  fn execute(&self) -> Result<(), AppError> {
    let command_output_result = Command::new("cargo")
      .arg("lambda build --release --target x86_64-unknown-linux-gnu")
      .output();

    if let Err(error) = command_output_result {
      return Err(AppError::Error(error.to_string()));
    }

    let command_output = command_output_result.unwrap();

    if command_output.status.success() {
      println!("Lambda project built successfully");
    }

    Ok(())
  }
}
