use std::fmt;
use std::process::exit;

#[derive(Debug)] // Arr&aw display with the format `{:?}`
pub enum ErrCode {
  ArgumentInvalid(&'static str),
}

impl ErrCode {
  // Return the exit code for the error
  pub fn get_retcode(&self) -> i32 {
    1
  }
}

#[allow(unreachable_patterns)]
impl fmt::Display for ErrCode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self {
      // Show argument error
      ErrCode::ArgumentInvalid(elem) => write!(f, "Invalid argument: {}", elem),

      _ => write!(f, "{:?}", self),
    }
  }
}

pub fn exit_with_retcode(res: Result<(), ErrCode>) {
  match res {
    Ok(_) => {
      let ret: i32 = 0;
      log::debug!("Exit without any error, returning {}", ret);
      exit(ret);
    }
    Err(err) => {
      let ret: i32 = err.get_retcode();
      log::error!("Error on exit:\n\t{}\n\tReturning {}", err, ret);
      exit(ret);
    }
  }
}
