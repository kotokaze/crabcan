use crate::errors::ErrCode;
use crate::ipc::create_socketpair;

use std::ffi::CString;
use std::os::unix::io::RawFd;
use std::path::PathBuf;

#[derive(Clone)]
pub struct ContainerOpts {
  pub path: CString,
  pub argv: Vec<CString>,

  pub fd: RawFd,
  pub uid: u32,
  pub mount_dir: PathBuf,
}

impl ContainerOpts {
  pub fn new(
    command: String,
    uid: u32,
    mount_dir: PathBuf,
  ) -> Result<(Self, (RawFd, RawFd)), ErrCode> {
    let argv: Vec<CString> = command
      .split_ascii_whitespace()
      .map(|s| CString::new(s).expect("Failed to convert string to CString"))
      .collect();
    let path = argv[0].clone();
    let sockets = create_socketpair()?;

    Ok((
      Self {
        path,
        argv,

        fd: sockets.1,
        uid,
        mount_dir,
      },
      sockets,
    ))
  }
}
