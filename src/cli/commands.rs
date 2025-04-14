use crate::{core::{Error, ErrorKind, Result}, types::utils::file_util::FerrumConfig, utils::{CommandUtil, CommandUtilProvider, FileUtil, FileUtilProvider}};

pub fn handle_build(_matches: &clap::ArgMatches) -> Result<()> {
  todo!();
}

/// Handles the deployment process for a Rust Lambda function.
pub fn handle_deploy(matches: &clap::ArgMatches) -> Result<()> {
  let bin_name = matches.get_one::<String>("bin_name")
    .unwrap();
  let lambda_function_name = matches.get_one::<String>("lambda_name")
    .unwrap();
  let profile_name = matches.get_one::<String>("profile_name")
    .unwrap();

  println!("🗂️ Reading configuration from ferrum.json...");
  let file_util = FileUtil::new();
  let ferrum_config: FerrumConfig = file_util.read_json("ferrum.json")?;
  println!("✅ Configuration loaded successfully");

  let profile = ferrum_config.profiles
    .get(profile_name)
    .ok_or_else(|| Error::new(ErrorKind::ProfileNotFound, "Profile not found"))?;

  println!("🗂️ Reading environment variables from {}...", &profile.env_path);
  let env_variable_lines = file_util.read_lines(&profile.env_path)?;
  println!("✅ Environment variables loaded");

  let command_util = CommandUtil::new();

  println!("🛠️ Building lambda binary...");
  command_util.execute("cargo", "lambda", &[
    "build", "--release",
    "--bin", &bin_name,
    "--target", "x86_64-unknown-linux-gnu",
  ])?;
  println!("✅ Build successful");

  println!("⚙️ Preparing deployment for lambda function {}...", lambda_function_name);
  let mut deploy_args: Vec<&str> = vec![
    "deploy",
    "--binary-name", bin_name,
    lambda_function_name,
    "--region", &ferrum_config.aws.region,
    "--iam-role", &ferrum_config.aws.iam_role
  ];

  deploy_args.extend(env_variable_lines
    .iter()
    .flat_map(|line| ["--env-var", line.as_str()]));

  println!("🚀 Deploying lambda function {} to AWS region {}...", lambda_function_name, &ferrum_config.aws.region);
  command_util.execute("cargo", "lambda", &deploy_args)?;
  println!("✅ Deployment successful!");

  Ok(())
}
