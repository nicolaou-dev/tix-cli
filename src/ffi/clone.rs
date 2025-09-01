use crate::ffi::{TIX_OUT_OF_MEMORY, TIX_COMMAND_FAILED, TixError, tix_clone};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloneError {
    #[error(transparent)]
    TixError(#[from] TixError),

    #[error("Invalid argument")]
    InvalidArgument,
}

pub fn clone(repo_url: &str) -> Result<String, CloneError> {
    let c_url = std::ffi::CString::new(repo_url).map_err(|_| CloneError::InvalidArgument)?;
    
    let result = unsafe { tix_clone(c_url.as_ptr()) };

    match result {
        0 => Ok("Repository cloned successfully".to_string()),
        TIX_OUT_OF_MEMORY => Err(CloneError::TixError(TixError::OutOfMemory)),
        TIX_COMMAND_FAILED => Err(CloneError::TixError(TixError::CommandFailed)),
        _ => Err(CloneError::TixError(TixError::UnknownError)),
    }
}