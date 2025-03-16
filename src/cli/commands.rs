use crate::{
  core::AppError,
  features::build::actions::{
    BuildLambda, IBuildLambda,
    ZipBootstrapFiles, IZipBootstrapFiles,
  },
};

pub fn handle_build(_matches: &clap::ArgMatches) -> Result<(), AppError> {
  let build_lambda = BuildLambda::new();
  let zip_bootstrap_files = ZipBootstrapFiles::new();

  println!("Building...");
  build_lambda.execute()?;
  println!("Zipping...");
  zip_bootstrap_files.execute()?;

  Ok(())
}

pub fn handle_deploy(matches: &clap::ArgMatches) -> Result<(), AppError> {
  handle_build(matches)?;

  Ok(())
}
