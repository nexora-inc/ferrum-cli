use crate::{
  core::{Error, ErrorKind, Result},
  features::{
    build::actions::{
      BuildLambdaBinaryAction, BuildLambdaBinaryActionProvider
    }, config::actions::{
      LoadConfigurationAction, LoadConfigurationActionProvider
    }, deploy::actions::{
      DeployLambdaAction, DeployLambdaActionProvider,
      LoadEnvironmentVariablesAction, LoadEnvironmentVariablesActionProvider
    },
  }, utils::{CommandUtil, FileUtil}
};

/// Handles the building process for a Rust Lambda binary.
pub fn handle_build(_matches: &clap::ArgMatches) -> Result<()> {
  todo!();
}

/// Handles the deployment process for a Rust Lambda function.
pub fn handle_deploy(matches: &clap::ArgMatches) -> Result<()> {
  println!("Loading and injecting dependencies...");
  let file_util = FileUtil::new();
  let command_util = CommandUtil::new();
  let load_configuration = LoadConfigurationAction::new(&file_util);
  let load_environment_variables = LoadEnvironmentVariablesAction::new(&file_util);
  let build_lambda_binary = BuildLambdaBinaryAction::new(&command_util);
  let deploy_lambda = DeployLambdaAction::new(&command_util);

  let bin_name: &String = matches.get_one("bin_name").unwrap();
  let lambda_function_name: &String = matches.get_one("lambda_name").unwrap();
  let profile_name: &String = matches.get_one("profile_name").unwrap();

  let ferrum_config = load_configuration.execute()?;

  let profile = ferrum_config.profiles
    .get(profile_name)
    .ok_or_else(|| Error::new(ErrorKind::ProfileNotFound, "Profile not found"))?;

  let env_variable_lines = load_environment_variables.execute(&profile)?;

  build_lambda_binary.execute(&bin_name)?;
  deploy_lambda.execute(
    &lambda_function_name,
    &bin_name,
    &ferrum_config.aws,
    &env_variable_lines,
  )?;

  Ok(())
}
