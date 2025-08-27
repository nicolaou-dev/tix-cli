// FFI bindings to the tix library
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// Include auto-generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod error;
pub use error::TixError;

pub mod init;
pub use init::init;

pub mod config;
pub use config::{config_get, config_set};

pub mod project;
pub use project::switch;

pub mod add;
pub use add::add;
