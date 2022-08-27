use crate::cli::Args;
use crate::config::ContainerOpts;
use crate::errors::ErrCode;

pub struct Container {
  config: ContainerOpts,
}

impl Container {
  pub fn new(args: Args) -> Result<Self, ErrCode> {
    let config = ContainerOpts::new(args.command, args.uid, args.mount_dir)?;

    Ok(Self { config })
  }

  pub fn create(&mut self) -> Result<(), ErrCode> {
    log::debug!("Creation completed");
    Ok(())
  }

  pub fn clean_exit(&mut self) -> Result<(), ErrCode> {
    log::debug!("Cleaning container");
    Ok(())
  }
}

pub fn start(args: Args) -> Result<(), ErrCode> {
  let mut container = Container::new(args)?;
  if let Err(err) = container.create() {
    container.clean_exit()?;
    log::error!("Error while creating container\n\t{:?}", err);
    return Err(err);
  }

  log::debug!("Finished, cleaning & exit");
  container.clean_exit()
}
