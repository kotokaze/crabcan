mod cli;
mod errors;

use std::process::exit;

use errors::exit_with_retcode;

fn main() {
  match cli::parse_args() {
    Ok(args) => {
      log::info!("Args: {:?}", args);
      exit_with_retcode(Ok(()))
    }
    Err(err) => {
      log::error!("Error while parsing args\n\t{}", err);
      exit(err.get_retcode())
    }
  };
}
