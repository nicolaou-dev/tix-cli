use thiserror::Error;

use crate::ffi::{TIX_INVALID_PRIORITY, TIX_INVALID_TITLE, TixError, tix_add};

#[derive(Debug, Error)]
pub enum AddError {
    #[error(transparent)]
    TixError(#[from] TixError),

    #[error("Invalid argument")]
    InvalidArgument,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum Priority {
    #[value(name = "a", help = "High priority")]
    a = b'a',
    #[value(name = "b", help = "Medium priority")]
    b = b'b',
    #[value(name = "c", help = "Low priority")]
    c = b'c',
    #[value(name = "z", help = "Default priority")]
    z = b'z',

    #[value(name = "none", help = "Use default priority")]
    None = 0,
}
pub fn add(title: &str, body: Option<&str>, priority: Priority) -> Result<String, AddError> {
    // SAFETY: tix_add expects a null-terminated string for title and body
    let c_title = std::ffi::CString::new(title).map_err(|_| AddError::InvalidArgument)?;

    let c_body = match body {
        Some(desc) => std::ffi::CString::new(desc).map_err(|_| AddError::InvalidArgument)?,
        None => std::ffi::CString::new("").unwrap(),
    };

    let mut value_ptr = std::ptr::null_mut();

    let result = unsafe {
        tix_add(
            c_title.as_ptr(),
            c_body.as_ptr(),
            priority as u8,
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
            unsafe { libc::free(value_ptr as *mut libc::c_void) };
            Ok(format!("Created ticket: {value}"))
        }

        TIX_INVALID_PRIORITY => Err(AddError::TixError(TixError::InvalidPriority)),
        TIX_INVALID_TITLE => Err(AddError::TixError(TixError::InvalidTitle)),

        _ => Err(AddError::TixError(TixError::UnknownError)),
    }
}
