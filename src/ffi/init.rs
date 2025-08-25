use crate::ffi::{TIX_INIT_ACCESS_DENIED, TIX_INIT_WORKSPACE_CREATION_FAILED, tix_init};
use strum::Display;
use thiserror::Error;

/// Result of initializing a tix workspace
#[derive(Debug, Display)]
pub enum InitResult {
    #[strum(serialize = "Initialized empty tix repository")]
    Initialized,
    #[strum(serialize = "Reinitialized existing tix repository")]
    Reinitialized,
}

#[derive(Debug, Error)]
pub enum TixError {
    #[error("Out of memory")]
    OutOfMemory,
    #[error("Not a tix repository")]
    NotARepository,
    #[error("Command failed")]
    CommandFailed,
    #[error("File system error")]
    FileSystemError,
    #[error("Unknown error")]
    UnknownError,
    #[error("Failed to create tix workspace")]
    InitWorkspaceCreationFailed,
    #[error("Access denied")]
    InitAccessDenied,
    #[error("Invalid configuration key")]
    ConfigInvalidKey,
    #[error("Remote already exists")]
    RemoteAlreadyExists,
    #[error("Invalid remote name")]
    RemoteInvalidName,
    #[error("Project not found")]
    SwitchProjectNotFound,
    #[error("Project already exists")]
    SwitchProjectAlreadyExists,
    #[error("Already on the specified project")]
    SwitchAlreadyOnProject,
    #[error("Invalid priority value")]
    InvalidPriority,
    #[error("Invalid title")]
    InvalidTitle,
}

pub fn init() -> Result<InitResult, TixError> {
    // SAFETY: tix_init is a simple function that doesn't take any pointers
    // and only returns an integer status code
    let result = unsafe { tix_init() };

    match result {
        0 => Ok(InitResult::Initialized),
        1 => Ok(InitResult::Reinitialized),
        TIX_INIT_WORKSPACE_CREATION_FAILED => Err(TixError::InitWorkspaceCreationFailed),
        TIX_INIT_ACCESS_DENIED => Err(TixError::InitAccessDenied),
        _ => Err(TixError::UnknownError),
    }
}
