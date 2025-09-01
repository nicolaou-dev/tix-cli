use thiserror::Error;

use crate::ffi::{
    TIX_INVALID_PRIORITY, TIX_INVALID_TITLE, TIX_INVALID_STATUS, TixError, priority::Priority, status::Status, tix_add, tix_add_free,
};

#[derive(Debug, Error)]
pub enum AddError {
    #[error(transparent)]
    TixError(#[from] TixError),

    #[error("Invalid argument")]
    InvalidArgument,
}

pub fn add(title: &str, body: Option<&str>, priority: Priority, status: Option<Status>) -> Result<String, AddError> {
    // SAFETY: tix_add expects a null-terminated string for title and body
    let c_title = std::ffi::CString::new(title).map_err(|_| AddError::InvalidArgument)?;

    let c_body = match body {
        Some(desc) => std::ffi::CString::new(desc).map_err(|_| AddError::InvalidArgument)?,
        None => std::ffi::CString::new("").unwrap(),
    };

    let mut value_ptr = std::ptr::null_mut();

    let status_byte = match status {
        Some(s) => s as u8,
        None => 0, // Let tix lib handle default
    };

    let result = unsafe {
        tix_add(
            c_title.as_ptr(),
            c_body.as_ptr(),
            priority as u8,
            status_byte,
            &mut value_ptr,
        )
    };

    match result {
        0 => {
            if value_ptr.is_null() {
                return Err(AddError::TixError(TixError::UnknownError));
            }
            // SAFETY: value_ptr is guaranteed to be a valid null-terminated string
            let c_str = unsafe { std::ffi::CStr::from_ptr(value_ptr) };
            let value = c_str.to_string_lossy().to_string();

            // SAFETY: value_ptr was allocated by tix_add and must be freed
            unsafe { tix_add_free(value_ptr) };
            Ok(format!("Created ticket: {value}"))
        }

        TIX_INVALID_PRIORITY => Err(AddError::TixError(TixError::InvalidPriority)),
        TIX_INVALID_TITLE => Err(AddError::TixError(TixError::InvalidTitle)),
        TIX_INVALID_STATUS => Err(AddError::TixError(TixError::InvalidStatus)),

        _ => Err(AddError::TixError(TixError::UnknownError)),
    }
}
