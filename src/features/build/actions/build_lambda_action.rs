use crate::{
  core::AppError,
  features::command::action::{ExecuteCommand, IExecuteCommand}
};

pub trait IBuildLambda {
  fn execute(&self) -> Result<(), AppError>;
}

pub struct BuildLambda {
  execute_command: ExecuteCommand,
}

impl BuildLambda {
  pub fn new(execute_command: ExecuteCommand) -> Self {
    Self { execute_command }
  }
}

impl IBuildLambda for BuildLambda {
  fn execute(&self) -> Result<(), AppError> {
    println!("Executing command...");
    let command_output = self.execute_command.execute(
      "cargo",
      "lambda",
      &["build", "--release", "--target", "x86_64-unknown-linux-gnu"]
    )?;

    println!("Checking command status...");
    if !command_output.status.success() {
      let std_error = String::from_utf8_lossy(&command_output.stderr);
      return Err(AppError::CommandError(std_error.to_string()))
    }

    println!("Lambda project built successfully");
    Ok(())
  }
}
