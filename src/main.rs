use ferrum_cli::cli;

fn main() {
  if let Err(error) = cli::run() {
    eprintln!("\n\n{:#?}", error);
    std::process::exit(1);
  }
}

