use crate::{
  core::AppError,
  features::command::action::IExecuteCommand,
};

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait IBuildLambda {
  fn execute(&self) -> Result<(), AppError>;
}

pub struct BuildLambda {
  execute_command: Box<dyn IExecuteCommand>,
}

impl BuildLambda {
  pub fn new(execute_command: Box<dyn IExecuteCommand>) -> Self {
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

#[cfg(test)]
mod tests {
  use super::*;
  use std::os::unix::process::ExitStatusExt;
  use std::process::{ExitStatus, Output};

  use crate::features::command::action::execute_command_action::MockIExecuteCommand;

  #[test]
  fn test_build_lambda_execute_success() {
    // arrange
    let mut mocked_execute_command = MockIExecuteCommand::new();
    mocked_execute_command.expect_execute()
      .with(
        eq("cargo"),
        eq("lambda"),
        eq(["build", "--release", "--target", "x86_64-unknown-linux-gnu"])
      ).returning(|_, _, _| {
        Ok(Output {
          status: ExitStatus::from_raw(0),
          stdout: b"Mocked stdout".to_vec(),
          stderr: Vec::new()
        })
      });

    let build_lambda = BuildLambda::new(Box::new(mocked_execute_command));

    // act
    let build_lambda_result = build_lambda.execute();

    // assert
    assert!(build_lambda_result.is_ok())
  }
}
