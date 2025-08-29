use thiserror::Error;

use crate::ffi::{
    Priority, Status, TIX_INVALID_TICKET_ID, TIX_OUT_OF_MEMORY, TIX_TICKET_NOT_FOUND, Ticket,
    TixError, tix_show, tix_show_body, tix_show_body_free, tix_show_priority, tix_show_status,
    tix_show_title, tix_show_title_free,
};

#[derive(Debug, Error)]
pub enum ShowError {
    #[error(transparent)]
    TixError(#[from] TixError),

    #[error("Invalid argument")]
    InvalidArgument,
}

pub fn show(ticket_id: &str) -> Result<Ticket, ShowError> {
    let c_ticket_id = std::ffi::CString::new(ticket_id).map_err(|_| ShowError::InvalidArgument)?;
    let mut output = std::ptr::null_mut();

    let result = unsafe { tix_show(c_ticket_id.as_ptr(), &mut output) };
    // Implementation of the show command

    match result {
        0 => {
            if output.is_null() {
                return Err(ShowError::TixError(TixError::UnknownError));
            }

            // SAFETY: output is a valid pointer to a CTicket
            let c_ticket = unsafe { *output };

            let ticket = Ticket::from(c_ticket);

            // SAFETY: c_ticket was allocated by tix_show and it must be feed
            unsafe { crate::ffi::tix_show_free(output) };

            Ok(ticket)
        }
        TIX_OUT_OF_MEMORY => Err(ShowError::TixError(TixError::OutOfMemory)),
        TIX_INVALID_TICKET_ID => Err(ShowError::TixError(TixError::InvalidTicketId)),
        TIX_TICKET_NOT_FOUND => Err(ShowError::TixError(TixError::TicketNotFound)),
        _ => Err(ShowError::TixError(TixError::UnknownError)),
    }
}

pub fn show_title(ticket_id: &str) -> Result<String, ShowError> {
    let c_ticket_id = std::ffi::CString::new(ticket_id).map_err(|_| ShowError::InvalidArgument)?;
    let mut output = std::ptr::null_mut();

    let result = unsafe { tix_show_title(c_ticket_id.as_ptr(), &mut output) };

    match result {
        0 => {
            if output.is_null() {
                return Err(ShowError::TixError(TixError::UnknownError));
            }

            let title = unsafe { std::ffi::CStr::from_ptr(output) }
                .to_string_lossy()
                .to_string();

            unsafe { tix_show_title_free(output) };

            Ok(title)
        }
        TIX_OUT_OF_MEMORY => Err(ShowError::TixError(TixError::OutOfMemory)),
        TIX_INVALID_TICKET_ID => Err(ShowError::TixError(TixError::InvalidTicketId)),
        TIX_TICKET_NOT_FOUND => Err(ShowError::TixError(TixError::TicketNotFound)),
        _ => Err(ShowError::TixError(TixError::UnknownError)),
    }
}

pub fn show_body(ticket_id: &str) -> Result<String, ShowError> {
    let c_ticket_id = std::ffi::CString::new(ticket_id).map_err(|_| ShowError::InvalidArgument)?;
    let mut output = std::ptr::null_mut();

    let result = unsafe { tix_show_body(c_ticket_id.as_ptr(), &mut output) };

    match result {
        0 => {
            if output.is_null() {
                return Err(ShowError::TixError(TixError::UnknownError));
            }

            let body = unsafe { std::ffi::CStr::from_ptr(output) }
                .to_string_lossy()
                .to_string();

            unsafe { tix_show_body_free(output) };

            Ok(body)
        }
        TIX_OUT_OF_MEMORY => Err(ShowError::TixError(TixError::OutOfMemory)),
        TIX_INVALID_TICKET_ID => Err(ShowError::TixError(TixError::InvalidTicketId)),
        TIX_TICKET_NOT_FOUND => Err(ShowError::TixError(TixError::TicketNotFound)),
        _ => Err(ShowError::TixError(TixError::UnknownError)),
    }
}

pub fn show_status(ticket_id: &str) -> Result<Status, ShowError> {
    let c_ticket_id = std::ffi::CString::new(ticket_id).map_err(|_| ShowError::InvalidArgument)?;

    let result = unsafe { tix_show_status(c_ticket_id.as_ptr()) };

    if result < 0 {
        match result {
            TIX_INVALID_TICKET_ID => Err(ShowError::TixError(TixError::InvalidTicketId)),
            TIX_TICKET_NOT_FOUND => Err(ShowError::TixError(TixError::TicketNotFound)),
            _ => Err(ShowError::TixError(TixError::UnknownError)),
        }
    } else {
        Ok(Status::from(result as u8))
    }
}

pub fn show_priority(ticket_id: &str) -> Result<Priority, ShowError> {
    let c_ticket_id = std::ffi::CString::new(ticket_id).map_err(|_| ShowError::InvalidArgument)?;

    let result = unsafe { tix_show_priority(c_ticket_id.as_ptr()) };

    if result < 0 {
        match result {
            TIX_INVALID_TICKET_ID => Err(ShowError::TixError(TixError::InvalidTicketId)),
            TIX_TICKET_NOT_FOUND => Err(ShowError::TixError(TixError::TicketNotFound)),
            _ => Err(ShowError::TixError(TixError::UnknownError)),
        }
    } else {
        Ok(Priority::from(result as u8))
    }
}
