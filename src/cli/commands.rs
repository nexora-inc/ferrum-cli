use crate::{
  core::AppError,
  features::build::actions::{
    BuildLambda, IBuildLambda,
    GetBuiltLambdaNames, IGetBuiltLambdaNames,
  },
};

pub fn handle_build(_matches: &clap::ArgMatches) -> Result<(), AppError> {
  println!("Building...");
  let build_lambda = BuildLambda::new();
  let get_built_lambda_names = GetBuiltLambdaNames::new();

  build_lambda.execute()?;
  let lambda_names = get_built_lambda_names.execute()?;

  println!("{:#?}", lambda_names);

  Ok(())
}

pub fn handle_deploy(_matches: &clap::ArgMatches) -> Result<(), AppError> {
  println!("Deploying...");
  let build_lambda = BuildLambda::new();
  let get_built_lambda_names = GetBuiltLambdaNames::new();

  build_lambda.execute()?;
  let lambda_names = get_built_lambda_names.execute()?;

  println!("{:#?}", lambda_names);

  Ok(())
}
