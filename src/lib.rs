#![allow(
    clippy::missing_safety_doc,
    clippy::missing_transmute_annotations,
    clippy::useless_transmute,
    clippy::upper_case_acronyms
)]

pub mod fxc;

mod blend_state;
mod buffer;
mod common;
mod context;
mod device;
mod device_child;
pub mod dxgi;
mod error;
mod input_layout;
mod query;
mod rasterizer;
mod resource;
mod sampler;
mod shader;
mod texture;
mod util;

mod dsv;
mod rtv;
mod srv;
mod uav;

pub use blend_state::*;
pub use buffer::*;
pub use common::*;
pub use context::*;
pub use device::*;
pub use device_child::*;
pub use dsv::*;
pub use error::*;
pub use input_layout::*;
pub use query::*;
pub use rasterizer::*;
pub use resource::*;
pub use rtv::*;
pub use sampler::*;
pub use shader::*;
pub use srv::*;
pub use texture::*;
pub use uav::*;

pub use d3d11_ffi as sys;
pub use d3d11_ffi::Foundation::HWND;
