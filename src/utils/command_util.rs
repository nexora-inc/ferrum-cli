use std::process::{Command, Output};

use crate::core::Result;

#[cfg(test)]
use mockall::{automock, predicate::*};

/// Provides functionalities for executing external commands.
#[cfg_attr(test, automock)]
pub trait CommandUtilProvider {
  /// Executes a command with the given program name, an optional subcommand,
  /// and additional arguments.
  ///
  /// # Arguments
  ///
  /// * `program`: The name or path of the executable program.
  /// * `subcommand`: An optional subcommand to pass immediately after the
  ///                 program name.
  /// * `arguments`: A slice of string arguments to pass to the program after
  ///                the subcommand.
  ///
  /// # Returns
  ///
  /// A `Result` containing the `Output` (stdout, stderr, status) of the
  /// executed command if successful. If the command fails to start or run
  /// (e.g., program not found, permissions error), an appropriate error
  /// variant is returned via the `crate::core::Result` type.
  fn execute<'a>(
    &self, program: &'a str,
    subcommand: &'a str,
    arguments: &'a [&'a str]
  ) -> Result<Output>;
}

pub struct CommandUtil;

impl CommandUtil {
  /// Creates a new instance of the `CommandUtil`.
  pub fn new() -> Self {
    Self {}
  }
}

impl CommandUtilProvider for CommandUtil {
  /// Executes the specified command using `std::process::Command`.
  fn execute<'a>(
    &self, program: &'a str,
    subcommand: &'a str,
    arguments: &'a [&'a str]
  ) -> Result<Output> {
    let mut command = Command::new(program);

    if !subcommand.is_empty() {
      command.arg(subcommand);
    }

    command.args(arguments);

    Ok(command.output()?)
  }
}
