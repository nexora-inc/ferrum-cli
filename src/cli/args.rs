use clap::Command;

pub fn build_cli() -> Command {
  Command::new("ferrum")
    .about("A simple CLI tool for ferrum projects")
    .subcommand(Command::new("new")
      .about("Create new ferrum project using cargo."))
    .subcommand(Command::new("build")
      .about("Build ferrum project using cargo."))
    .subcommand(Command::new("deploy")
      .about("Deploy ferrum project using serverless framework."))
}
