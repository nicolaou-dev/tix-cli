use crate::ffi::{TIX_NOT_A_REPOSITORY, TIX_COMMAND_FAILED, TixError, tix_push};

pub fn push() -> Result<String, TixError> {
    let result = unsafe { tix_push() };

    match result {
        0 => Ok("Pushed changes to remote".to_string()),
        TIX_NOT_A_REPOSITORY => Err(TixError::NotARepository),
        TIX_COMMAND_FAILED => Err(TixError::CommandFailed),
        _ => Err(TixError::UnknownError),
    }
}