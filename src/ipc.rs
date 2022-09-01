use crate::errors::ErrCode;

use nix::sys::socket::{recv, send, socketpair, AddressFamily, MsgFlags, SockFlag, SockType};
use std::os::unix::io::RawFd;

pub fn create_socketpair() -> Result<(RawFd, RawFd), ErrCode> {
  match socketpair(
    AddressFamily::Unix,
    SockType::SeqPacket,
    None,
    SockFlag::SOCK_CLOEXEC,
  ) {
    Ok(res) => Ok(res),
    Err(_) => Err(ErrCode::SocketError(0)),
  }
}

pub fn send_boolean(fd: RawFd, val: bool) -> Result<(), ErrCode> {
  if let Err(err) = send(fd, &[val.into(); 1usize], MsgFlags::empty()) {
    log::error!("Cannot send boolean through socket: {:?}", err);
    return Err(ErrCode::SocketError(1));
  }

  Ok(())
}

pub fn recv_boolean(fd: RawFd) -> Result<bool, ErrCode> {
  let mut buf = [0u8; 1usize];
  if let Err(err) = recv(fd, &mut buf, MsgFlags::empty()) {
    log::error!("Cannot receive boolean through socket: {:?}", err);
    return Err(ErrCode::SocketError(2));
  }

  Ok(buf[0] != 0)
}
