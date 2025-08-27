use thiserror::Error;

use crate::ffi::{TIX_CONFIG_INVALID_KEY, TixError, tix_config_set};

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
