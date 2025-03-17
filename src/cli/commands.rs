use crate::{
  core::AppError,
  features::{
    build::actions::{
      BuildLambda, IBuildLambda, IZipBootstrapFiles, ZipBootstrapFiles
    }, command::action::ExecuteCommand, deploy::actions::{
      IServerlessDeploy, ServerlessDeploy
    }
  },
};

pub fn handle_build(_matches: &clap::ArgMatches) -> Result<(), AppError> {
  let execute_command = ExecuteCommand::new();
  let build_lambda = BuildLambda::new(execute_command);
  let zip_bootstrap_files = ZipBootstrapFiles::new();

  println!("Building...");
  build_lambda.execute()?;
  println!("Zipping...");
  zip_bootstrap_files.execute()?;

  Ok(())
}

pub fn handle_deploy(matches: &clap::ArgMatches) -> Result<(), AppError> {
  let serverless_deploy = ServerlessDeploy::new();

  handle_build(matches)?;
  serverless_deploy.execute()?;

  Ok(())
}
