use crate::config::ContainerOpts;
use crate::errors::ErrCode;

use nix::sched::{clone, CloneFlags};
use nix::sys::signal::Signal;
use nix::unistd::Pid;

const STACK_SIZE: usize = 1024 * 1024; // 1KiB

fn child(config: ContainerOpts) -> isize {
  log::info!(
    "Starting container with command {} and args {:?}",
    config.path.to_str().unwrap(),
    config.argv
  );
  0
}

pub fn cerate_child_process(config: ContainerOpts) -> Result<Pid, ErrCode> {
  let mut flags = CloneFlags::empty();
  flags.insert(CloneFlags::CLONE_NEWNS);
  flags.insert(CloneFlags::CLONE_NEWCGROUP);
  flags.insert(CloneFlags::CLONE_NEWPID);
  flags.insert(CloneFlags::CLONE_NEWIPC);
  flags.insert(CloneFlags::CLONE_NEWNET);
  flags.insert(CloneFlags::CLONE_NEWUTS);

  match clone(
    Box::new(|| child(config.clone())),
    &mut [0u8; STACK_SIZE],
    flags,
    Some(Signal::SIGCHLD as i32),
  ) {
    Ok(pid) => Ok(pid),
    Err(_) => Err(ErrCode::ChildProcessError(0)),
  }
}
