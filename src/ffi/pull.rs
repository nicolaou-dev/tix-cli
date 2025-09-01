use crate::ffi::{TIX_NOT_A_REPOSITORY, TIX_COMMAND_FAILED, TixError, tix_pull};

pub fn pull() -> Result<String, TixError> {
    let result = unsafe { tix_pull() };

    match result {
        0 => Ok("Pulled changes from remote".to_string()),
        TIX_NOT_A_REPOSITORY => Err(TixError::NotARepository),
        TIX_COMMAND_FAILED => Err(TixError::CommandFailed),
        _ => Err(TixError::UnknownError),
    }
}