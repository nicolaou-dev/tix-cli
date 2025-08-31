use crate::ffi::{TIX_NOT_A_REPOSITORY, TIX_COMMAND_FAILED, TixError, tix_redo};

pub fn redo() -> Result<String, TixError> {
    // SAFETY: tix_redo is a simple function that doesn't take any pointers
    // and only returns an integer status code
    let result = unsafe { tix_redo() };

    match result {
        0 => Ok("Redid last undone change".to_string()),
        TIX_NOT_A_REPOSITORY => Err(TixError::NotARepository),
        TIX_COMMAND_FAILED => Err(TixError::CommandFailed),
        _ => Err(TixError::UnknownError),
    }
}