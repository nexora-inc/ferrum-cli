use ferrum_cli::cli;

fn main() {
  if let Err(error) = cli::run() {
    eprintln!("\n\n\n\n\nError: {:#?}\n\n\n\n\n", error);
    std::process::exit(1);
  }
}

