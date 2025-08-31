use crate::ffi::{TIX_NOT_A_REPOSITORY, TIX_COMMAND_FAILED, TixError, tix_log, tix_log_free};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LogError {
    #[error(transparent)]
    TixError(#[from] TixError),

    #[error("Invalid argument")]
    InvalidArgument,
}

pub fn log(oneline: bool, limit: Option<i32>, since: Option<&str>) -> Result<String, LogError> {
    let oneline_flag = if oneline { 1 } else { 0 };
    let limit_value = limit.unwrap_or(0);
    
    let c_since = if let Some(since_str) = since {
        Some(std::ffi::CString::new(since_str).map_err(|_| LogError::InvalidArgument)?)
    } else {
        None
    };

    let since_ptr = c_since.as_ref().map_or(std::ptr::null(), |s| s.as_ptr());

    let mut output_ptr = std::ptr::null_mut();
    let result = unsafe { tix_log(&mut output_ptr, oneline_flag, limit_value, since_ptr) };

    match result {
        0 => {
            if output_ptr.is_null() {
                return Err(LogError::TixError(TixError::UnknownError));
            }
            // SAFETY: output_ptr is guaranteed to be a valid null-terminated string
            let c_str = unsafe { std::ffi::CStr::from_ptr(output_ptr) };
            let output = c_str.to_string_lossy().to_string();

            // SAFETY: output_ptr was allocated by tix_log and must be freed
            unsafe { tix_log_free(output_ptr) };
            Ok(output)
        }
        TIX_NOT_A_REPOSITORY => Err(LogError::TixError(TixError::NotARepository)),
        TIX_COMMAND_FAILED => Err(LogError::TixError(TixError::CommandFailed)),
        _ => Err(LogError::TixError(TixError::UnknownError)),
    }
}