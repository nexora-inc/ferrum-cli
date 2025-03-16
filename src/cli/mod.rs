pub mod args;
pub mod commands;

use crate::core::AppError;

pub fn run() -> Result<(), AppError> {
  let matches = args::build_cli().get_matches();

  if let Some(matches) = matches.subcommand_matches("build") {
    commands::handle_build(matches)?
  } else if let Some(matches) = matches.subcommand_matches("deploy") {
    commands::handle_deploy(matches)?
  }

 Ok(())
}
