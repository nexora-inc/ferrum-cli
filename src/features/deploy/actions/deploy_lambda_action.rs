use crate::{core::Result, types::utils::file_util::FerrumConfigAws, utils::CommandUtilProvider};

pub trait DeployLambdaActionProvider {
  fn execute<'a>(
    &self, lambda_function_name: &str,
    bin_name: &str,
    ferrum_config_aws: &FerrumConfigAws,
    env_variable_lines: &'a Vec<String>,
  ) -> Result<()>;
}

pub struct DeployLambdaAction<'a, CommandUtil: CommandUtilProvider> {
  command_util: &'a CommandUtil,
}

impl<'a, CommandUtil: CommandUtilProvider> DeployLambdaAction<'a, CommandUtil> {
  pub fn new(command_util: &'a CommandUtil) -> Self { Self { command_util } }
}

impl<'a, CommandUtil: CommandUtilProvider> DeployLambdaActionProvider
  for DeployLambdaAction<'a, CommandUtil> {
  fn execute<'b>(
    &self, lambda_function_name: &str,
    bin_name: &str,
    ferrum_config_aws: &FerrumConfigAws,
    env_variable_lines: &'b Vec<String>,
  ) -> Result<()> {
    println!("⚙️ Preparing deployment for lambda function {}...", lambda_function_name);
    let mut deploy_args: Vec<&str> = vec![
      "deploy",
      "--binary-name", bin_name,
      lambda_function_name,
      "--region", &ferrum_config_aws.region,
      "--iam-role", &ferrum_config_aws.iam_role
    ];

    deploy_args.extend(env_variable_lines
      .iter()
      .flat_map(|line| ["--env-var", line.as_str()]));

    println!("🚀 Deploying lambda function {} to AWS region {}...", lambda_function_name, &ferrum_config_aws.region);
    self.command_util.execute("cargo", "lambda", &deploy_args)?;
    println!("✅ Deployment successful!");

    Ok(())
  }
}
