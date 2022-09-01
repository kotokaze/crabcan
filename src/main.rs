mod child;
mod cli;
mod config;
mod container;
mod errors;
mod ipc;

use errors::exit_with_retcode;

#[macro_use]
extern crate scan_fmt;

use std::process::exit;

fn main() {
  match cli::parse_args() {
    Ok(args) => {
      log::info!("Args: {:?}", args);
      exit_with_retcode(container::start(args));
    }
    Err(err) => {
      log::error!("Error while parsing args\n\t{}", err);
      exit(err.get_retcode());
    }
  };
}
