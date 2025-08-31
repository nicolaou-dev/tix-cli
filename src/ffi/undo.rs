use crate::ffi::{TIX_NOT_A_REPOSITORY, TIX_COMMAND_FAILED, TixError, tix_undo};

pub fn undo() -> Result<String, TixError> {
    // SAFETY: tix_undo is a simple function that doesn't take any pointers
    // and only returns an integer status code
    let result = unsafe { tix_undo() };

    match result {
        0 => Ok("Undid last change".to_string()),
        TIX_NOT_A_REPOSITORY => Err(TixError::NotARepository),
        TIX_COMMAND_FAILED => Err(TixError::CommandFailed),
        _ => Err(TixError::UnknownError),
    }
}