use std::ffi::CString;

use thiserror::Error;

use crate::ffi::{
    Priority, Status, TIX_INVALID_PRIORITY, TIX_INVALID_STATUS, Ticket, TixError,
    priority::PriorityVecExt, status::StatusVecExt, tix_list, tix_list_free,
};

#[derive(Debug, Error)]
pub enum ListError {
    #[error(transparent)]
    TixError(#[from] TixError),

    #[error("Invalid argument")]
    InvalidArgument,
    #[error("Invalid status")]
    InvalidStatus,
}

pub fn list(
    _long: bool,
    statuses: Vec<Status>,
    priorities: Vec<Priority>,
) -> Result<Vec<Ticket>, ListError> {
    let c_key = CString::new(statuses.to_bytes()).map_err(|_| ListError::InvalidArgument)?;

    let c_val = CString::new(priorities.to_bytes()).map_err(|_| ListError::InvalidArgument)?;

    let mut output = std::ptr::null_mut();
    let mut count: usize = 0;
    let result = unsafe { tix_list(c_key.as_ptr(), c_val.as_ptr(), &mut output, &mut count) };

    match result {
        0 => {
            if output.is_null() {
                return Err(ListError::TixError(TixError::UnknownError));
            }

            if count == 0 {
                return Ok(vec![]);
            }

            // SAFETY: output is a valid pointer to an array of CTicket of length count
            let tickets: Vec<_> = unsafe {
                std::slice::from_raw_parts(output, count)
                    .iter()
                    .map(|&t| Ticket::from(t))
                    .collect()
            };

            // SAFETY: ticket_list was allocated by tix_list and must be freed
            unsafe {
                tix_list_free(output, count);
            };
            Ok(tickets)
        }
        TIX_INVALID_PRIORITY => Err(ListError::TixError(TixError::InvalidPriority)),
        TIX_INVALID_STATUS => Err(ListError::TixError(TixError::InvalidStatus)),
        _ => Err(ListError::TixError(TixError::UnknownError)),
    }
}
