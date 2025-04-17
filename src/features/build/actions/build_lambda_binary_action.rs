use crate::{core::Result, utils::CommandUtilProvider};

pub trait BuildLambdaBinaryActionProvider {
  fn execute(&self, bin_name: &str) -> Result<()>;
}

pub struct BuildLambdaBinaryAction<'a, CommandUtil: CommandUtilProvider> {
  command_util: &'a CommandUtil,
}

impl<'a, CommandUtil: CommandUtilProvider>
  BuildLambdaBinaryAction<'a, CommandUtil> {
  pub fn new(command_util: &'a CommandUtil) -> Self { Self { command_util } }
}

impl<'a, CommandUtil: CommandUtilProvider> BuildLambdaBinaryActionProvider
  for BuildLambdaBinaryAction<'a, CommandUtil> {
  fn execute(&self, bin_name: &str) -> Result<()> {
    println!("🛠️ Building lambda binary...");
    self.command_util.execute("cargo", "lambda", &[
      "build", "--release",
      "--bin", bin_name,
      "--target", "x86_64-unknown-linux-gnu",
    ])?;
    println!("✅ Build successful");

    Ok(())
  }
}
