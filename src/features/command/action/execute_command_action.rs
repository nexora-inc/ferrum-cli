use std::process::{Command, Output};

use crate::core::AppError;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait IExecuteCommand {
  fn execute(
    &self, program_name: &'static str,
    subcommand: &'static str,
    arguments: &[&'static str]
  ) -> Result<Output, AppError>;
}

pub struct ExecuteCommand;

impl ExecuteCommand {
  pub fn new() -> Self {
    Self {}
  }
}

impl IExecuteCommand for ExecuteCommand {
  fn execute(&self, program_name: &'static str, subcommand: &'static str, arguments: &[&'static str]) -> Result<Output, AppError> {
    Ok(Command::new(program_name)
      .arg(subcommand)
      .args(arguments)
      .output()?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::os::unix::process::ExitStatusExt;
  use std::process::{ExitStatus, Output};

  #[test]
  fn test_execute_command_success() {
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

      // Act
    let execution_result = mocked_execute_command.execute(
      "cargo",
      "lambda",
      &["build", "--release", "--target", "x86_64-unknown-linux-gnu"],
    );

    // Assert
    assert!(execution_result.is_ok());
    let command_output = execution_result.unwrap();
    assert!(command_output.status.success());
  }
}
