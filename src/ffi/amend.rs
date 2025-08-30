use thiserror::Error;

use crate::ffi::{Priority, TIX_INVALID_PRIORITY, TIX_INVALID_STATUS, TixError, tix_amend};

#[derive(Debug, Error)]
pub enum AmendError {
    #[error(transparent)]
    TixError(#[from] TixError),

    #[error("Invalid argument")]
    InvalidArgument,
}

pub fn amend(
    ticket_id: &str,
    title: Option<&str>,
    body: Option<&str>,
    priority: Option<Priority>,
) -> Result<(), AmendError> {
    let c_ticket_id = std::ffi::CString::new(ticket_id).map_err(|_| AmendError::InvalidArgument)?;
    let c_title = match title {
        Some(t) => std::ffi::CString::new(t).map_err(|_| AmendError::InvalidArgument)?,
        None => std::ffi::CString::new("").unwrap(),
    };
    let c_body = match body {
        Some(desc) => std::ffi::CString::new(desc).map_err(|_| AmendError::InvalidArgument)?,
        None => std::ffi::CString::new("").unwrap(),
    };

    let result = unsafe {
        tix_amend(
            c_ticket_id.as_ptr(),
            c_title.as_ptr(),
            c_body.as_ptr(),
            priority.map(|p| p as u8).unwrap_or(0),
        )
    };

    match result {
        0 => Ok(()),
        TIX_INVALID_PRIORITY => Err(AmendError::TixError(TixError::InvalidPriority)),
        TIX_INVALID_STATUS => Err(AmendError::TixError(TixError::InvalidStatus)),
        _ => Err(AmendError::TixError(TixError::UnknownError)),
    }
}
