use crate::ffi::{TIX_NOT_A_REPOSITORY, TIX_COMMAND_FAILED, TixError, tix_remote, tix_remote_free};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RemoteError {
    #[error(transparent)]
    TixError(#[from] TixError),

    #[error("Invalid argument")]
    InvalidArgument,
}

pub fn remote(verbose: bool) -> Result<String, RemoteError> {
    let verbose_flag = if verbose { 1 } else { 0 };
    let mut output_ptr = std::ptr::null_mut();
    
    let result = unsafe { tix_remote(verbose_flag, &mut output_ptr) };

    match result {
        0 => {
            if output_ptr.is_null() {
                return Ok(String::new());
            }
            let c_str = unsafe { std::ffi::CStr::from_ptr(output_ptr) };
            let output = c_str.to_string_lossy().to_string();

            unsafe { tix_remote_free(output_ptr) };
            Ok(output)
        }
        TIX_NOT_A_REPOSITORY => Err(RemoteError::TixError(TixError::NotARepository)),
        TIX_COMMAND_FAILED => Err(RemoteError::TixError(TixError::CommandFailed)),
        _ => Err(RemoteError::TixError(TixError::UnknownError)),
    }
}