use crate::ffi::{CTicket, Priority, Status};

pub struct Ticket {
    pub id: String,
    pub title: String,
    pub body: Option<String>,
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
            // if body is not null convert it to String, otherwise set it to None
            body: if c_ticket.body.is_null() {
                None
            } else {
                Some(
                    // SAFETY: c_ticket.body is a valid null-terminated C string
                    unsafe { std::ffi::CStr::from_ptr(c_ticket.body) }
                        .to_string_lossy()
                        .to_string(),
                )
            },
            priority: Priority::from(c_ticket.priority),
            status: Status::from(c_ticket.status),
        }
    }
}
