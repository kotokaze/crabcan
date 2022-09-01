use crate::child::cerate_child_process;
use crate::cli::Args;
use crate::config::ContainerOpts;
use crate::errors::ErrCode;

use nix::sys::utsname::{uname, UtsName};
use nix::sys::wait::waitpid;
use nix::unistd::{close, Pid};
use std::os::unix::io::RawFd;

pub const MIN_KERNEL_VERSION: f32 = 4.8;

pub struct Container {
  sockets: (RawFd, RawFd),
  config: ContainerOpts,
  child_pid: Option<Pid>,
}

impl Container {
  pub fn new(args: Args) -> Result<Self, ErrCode> {
    let (config, sockets) = ContainerOpts::new(args.command, args.uid, args.mount_dir)?;

    Ok(Self {
      sockets,
      config,
      child_pid: None,
    })
  }

  pub fn create(&mut self) -> Result<(), ErrCode> {
    let pid: Pid = cerate_child_process(self.config.clone())?;
    self.child_pid = Some(pid);
    log::debug!("Creation completed");
    Ok(())
  }

  pub fn clean_exit(&mut self) -> Result<(), ErrCode> {
    log::debug!("Cleaning container");

    if let Err(err) = close(self.sockets.0) {
      log::error!("Unable to close write socket: {:?}", err);
      return Err(ErrCode::SocketError(3));
    }

    if let Err(err) = close(self.sockets.1) {
      log::error!("Unable to close write socket: {:?}", err);
      return Err(ErrCode::SocketError(4));
    }

    Ok(())
  }
}

pub fn check_host() -> Result<(), ErrCode> {
  let host: UtsName = uname();
  log::debug!("Host kernel version: {}", host.release());

  if let Ok(version) = scan_fmt!(host.release(), "{f}.{}", f32) {
    if version < MIN_KERNEL_VERSION {
      return Err(ErrCode::NotSupported(0));
    }
  } else {
    return Err(ErrCode::ContainerError(0));
  }

  if host.machine() != "x86_64" {
    return Err(ErrCode::NotSupported(1));
  }

  Ok(())
}

pub fn wait_child(pid: Option<Pid>) -> Result<(), ErrCode> {
  if let Some(child_pid) = pid {
    log::debug!("Waiting for child (pid {}) to finish", child_pid);
    if let Err(err) = waitpid(child_pid, None) {
      log::error!("Error while waiting for pid to finish: {:?}", err);
      return Err(ErrCode::ContainerError(1));
    }
  }
  Ok(())
}

pub fn start(args: Args) -> Result<(), ErrCode> {
  check_host()?;

  let mut container = Container::new(args)?;
  log::debug!("Container sockets: ({}, {})", container.sockets.0, container.sockets.1);

  if let Err(err) = container.create() {
    container.clean_exit()?;
    log::error!("Error while creating container\n\t{:?}", err);
    return Err(err);
  }

  log::debug!("Container child PID: {:?}", container.child_pid);
  wait_child(container.child_pid)?;

  log::debug!("Finished, cleaning & exit");
  container.clean_exit()
}
