// FFI bindings to the tix library
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// Include auto-generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod init;
pub use init::init;
