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

pub fn parse_args() -> Args {
  let args: Args = Args::from_args();

  /* Setup logging here */
  // IF args.debug: Setup log at debug level
  // ELSE: Setup log at info level

  /* Validate args and return them */
  args
}
