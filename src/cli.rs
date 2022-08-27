use crate::errors::ErrCode;

use log::LevelFilter;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "crabcan", about = "A simple container in Rust")]
pub struct Args {
  // Activate debug mode
  #[structopt(short, long)]
  debug: bool,

  // Command to execute inside the container
  #[structopt(short, long)]
  pub command: String,

  // User ID to create inside the container
  #[structopt(short, long)]
  pub uid: u32,

  // Directory to mount as a root of the container
  #[structopt(short = "m", long = "mount", parse(from_os_str))]
  pub mount_dir: PathBuf,
}

fn setup_log(level: LevelFilter) {
  let mut builder = env_logger::Builder::from_default_env();
  builder.format_timestamp_secs();
  builder.filter_level(level);
  builder.init();
}

pub fn parse_args() -> Result<Args, ErrCode> {
  let args: Args = Args::from_args();

  /* Setup logging */
  let level: LevelFilter = if args.debug {
    LevelFilter::Debug
  } else {
    LevelFilter::Info
  };
  setup_log(level);

  /* Validate args and return them */
  Ok(args)
}
