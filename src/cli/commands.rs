use std::fs;

use crate::{core::Result, types::utils::file_util::FerrumConfig, utils::{FileUtil, FileUtilProvider}};

pub fn handle_build(_matches: &clap::ArgMatches) -> Result<()> {
  todo!();
}

pub fn handle_deploy(matches: &clap::ArgMatches) -> Result<()> {
  let bin_name = matches.get_one::<String>("bin_name")
    .unwrap();
  let lambda_name = matches.get_one::<String>("lambda_name")
    .unwrap();
  let profile_name = matches.get_one::<String>("profile_name")
    .unwrap();

  println!("🗂️ Reading from ferrum.json for configuration...");
  let file_util = FileUtil::new();
  let data: FerrumConfig = file_util.read_json("ferrum.json")?;
  println!("{:?}", data);
  println!("✅ Done");

  let profile = data.profiles
    .get(profile_name)
    .unwrap();

  println!("🗂️ Reading from {} for environment variables...", &profile.env_path);
  let env_lines = file_util.read_lines(&profile.env_path);
  println!("✅ Done");

  println!("🛠️ Building...");
  println!("✅ Done");

  println!("⚙️ Deploying...");
  println!("✅ Done");

  Ok(())
}
