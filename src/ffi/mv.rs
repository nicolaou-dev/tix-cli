use thiserror::Error;

use crate::ffi::{Status, TIX_INVALID_STATUS, TIX_INVALID_TICKET_ID, TixError, tix_move};

#[derive(Debug, Error)]
pub enum MvError {
    #[error(transparent)]
    TixError(#[from] TixError),

    #[error("Invalid argument")]
    InvalidArgument,
    #[error("Invalid status")]
    InvalidStatus,
}
pub fn mv(ticket_id: &str, status: Status) -> Result<(), MvError> {
    let c_ticket_id = std::ffi::CString::new(ticket_id).map_err(|_| MvError::InvalidArgument)?;

    let result = unsafe { tix_move(c_ticket_id.as_ptr(), status as u8) };

    match result {
        0 => Ok(()),
        TIX_INVALID_TICKET_ID => Err(MvError::TixError(TixError::InvalidTicketId)),
        TIX_INVALID_STATUS => Err(MvError::InvalidStatus),
        _ => Err(MvError::TixError(TixError::UnknownError)),
    }
}
