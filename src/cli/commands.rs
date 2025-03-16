use crate::{
  core::AppError,
  features::build::actions::{
    BuildLambda, IBuildLambda,
    ZipBootstrapFiles, IZipBootstrapFiles,
  },
};

pub fn handle_build(_matches: &clap::ArgMatches) -> Result<(), AppError> {
  println!("Building...");
  let build_lambda = BuildLambda::new();
  let zip_bootstrap_files = ZipBootstrapFiles::new();

  build_lambda.execute()?;
  zip_bootstrap_files.execute()?;

  Ok(())
}

pub fn handle_deploy(_matches: &clap::ArgMatches) -> Result<(), AppError> {
  println!("Deploying...");
  let build_lambda = BuildLambda::new();

  build_lambda.execute()?;

  Ok(())
}
