#![feature(custom_inner_attributes)]
#![allow(
    soft_unstable,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    // cohae: We are not going to fix these, as most of the `windows` crate is generated code.
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
    clippy::missing_transmute_annotations,
)]

pub mod Direct3D;
pub mod Direct3D11;
pub mod Dxgi;
pub mod Foundation;
pub mod Security;

pub use windows_core as core;
