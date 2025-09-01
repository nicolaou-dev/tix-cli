use crate::ffi::{TIX_INIT_ACCESS_DENIED, TIX_INIT_WORKSPACE_CREATION_FAILED, TIX_INIT_NOT_ON_MAIN_BRANCH, TixError, tix_init};
use strum::Display;

/// Result of initializing a tix workspace
#[derive(Debug, Display)]
pub enum InitResult {
    #[strum(serialize = "Initialized empty tix repository")]
    Initialized,
    #[strum(serialize = "Reinitialized existing tix repository")]
    Reinitialized,
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
        TIX_INIT_NOT_ON_MAIN_BRANCH => Err(TixError::InitNotOnMain),
        _ => Err(TixError::UnknownError),
    }
}
