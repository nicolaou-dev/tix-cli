use crate::ffi::{TIX_NOT_A_REPOSITORY, TIX_COMMAND_FAILED, TIX_PUSH_REJECTED_NEEDS_FORCE, TixError, tix_push};

pub fn push(force: bool, force_with_lease: bool) -> Result<String, TixError> {
    let force_flag = if force { 1 } else { 0 };
    let force_with_lease_flag = if force_with_lease { 1 } else { 0 };
    
    let result = unsafe { tix_push(force_flag, force_with_lease_flag) };

    match result {
        0 => Ok("Pushed changes to remote".to_string()),
        TIX_NOT_A_REPOSITORY => Err(TixError::NotARepository),
        TIX_COMMAND_FAILED => Err(TixError::CommandFailed),
        TIX_PUSH_REJECTED_NEEDS_FORCE => Err(TixError::PushRejectedNeedsForce),
        _ => Err(TixError::UnknownError),
    }
}