use std::ffi::CString;

use thiserror::Error;

use crate::ffi::{
    CTicket, Priority, Status, TIX_INVALID_PRIORITY, TIX_INVALID_STATUS, TixError,
    priority::PriorityVecExt, status::StatusVecExt, tix_list,
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

pub struct Ticket {
    pub id: String,
    pub title: String,
    pub priority: Priority,
    pub status: Status,
}

impl From<CTicket> for Ticket {
    fn from(c_ticket: CTicket) -> Self {
        Ticket {
            // SAFETY: c_ticket.id is a valid null-terminated C string
            id: unsafe { std::ffi::CStr::from_ptr(c_ticket.id) }
                .to_string_lossy()
                .to_string(),
            // SAFETY: c_ticket.title is a valid null-terminated C string
            title: unsafe { std::ffi::CStr::from_ptr(c_ticket.title) }
                .to_string_lossy()
                .to_string(),
            priority: Priority::from(c_ticket.priority),
            status: Status::from(c_ticket.status),
        }
    }
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
                for i in 0..count {
                    let ct = output.add(i);
                    libc::free((*ct).title as *mut _);
                    libc::free((*ct).id as *mut _);
                }
                libc::free(output as *mut _)
            };
            Ok(tickets)
        }
        TIX_INVALID_PRIORITY => Err(ListError::TixError(TixError::InvalidPriority)),
        TIX_INVALID_STATUS => Err(ListError::TixError(TixError::InvalidStatus)),
        _ => Err(ListError::TixError(TixError::UnknownError)),
    }
}
