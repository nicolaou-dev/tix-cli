use thiserror::Error;

use crate::ffi::{TIX_CONFIG_INVALID_KEY, TixError, tix_config_get_free, tix_config_set};

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error(transparent)]
    TixError(#[from] TixError),

    #[error("Invalid argument")]
    InvalidArgument,
}

pub fn config_set(key: &str, value: &str) -> Result<(), ConfigError> {
    // SAFETY: tix_set_config expects two null-terminated strings
    let c_key = std::ffi::CString::new(key).map_err(|_| ConfigError::InvalidArgument)?;
    let c_value = std::ffi::CString::new(value).map_err(|_| ConfigError::InvalidArgument)?;

    let result = unsafe { tix_config_set(c_key.as_ptr(), c_value.as_ptr()) };

    match result {
        0 => Ok(()),
        TIX_CONFIG_INVALID_KEY => Err(ConfigError::TixError(TixError::ConfigInvalidKey)),
        _ => Err(ConfigError::TixError(TixError::UnknownError)),
    }
}

// Use leaks --atExit -- tix config user.name on mac to check for memory leaks
// Commenting out libc::free causes a memory leak -> for testing purposes
pub fn config_get(key: &str) -> Result<String, ConfigError> {
    // SAFETY: tix_get_config expects a null-terminated string
    let c_key = std::ffi::CString::new(key).map_err(|_| ConfigError::InvalidArgument)?;

    // create a pointer to hold the result. This will be freed by rust
    let mut value_ptr = std::ptr::null_mut();
    let result = unsafe { crate::ffi::tix_config_get(c_key.as_ptr(), &mut value_ptr) };

    match result {
        0 => {
            if value_ptr.is_null() {
                return Err(ConfigError::TixError(TixError::UnknownError));
            }
            // SAFETY: value_ptr is guaranteed to be a valid null-terminated string
            let c_str = unsafe { std::ffi::CStr::from_ptr(value_ptr) };
            let value = c_str.to_string_lossy().to_string();

            // SAFETY: value_ptr was allocated by tix_config_get and must be freed
            unsafe { tix_config_get_free(value_ptr) };
            Ok(value)
        }
        TIX_CONFIG_INVALID_KEY => Err(ConfigError::TixError(TixError::ConfigInvalidKey)),
        _ => Err(ConfigError::TixError(TixError::UnknownError)),
    }
}
