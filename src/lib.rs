#![allow(
    clippy::missing_safety_doc,
    clippy::missing_transmute_annotations,
    clippy::useless_transmute,
    clippy::upper_case_acronyms
)]

pub mod blend_state;
pub mod buffer;
pub mod common;
pub mod context;
pub mod device;
pub mod device_child;
pub mod dxgi;
pub mod error;
pub mod input_layout;
pub mod query;
pub mod rasterizer;
pub mod resource;
pub mod sampler;
pub mod shader;
pub mod texture;
mod util;

pub mod dsv;
pub mod rtv;
pub mod srv;
pub mod uav;

pub use blend_state::{Blend, BlendDesc, BlendOp, BlendState, RenderTargetBlendDesc};
pub use buffer::{Buffer, BufferDesc};
pub use common::*;
pub use context::*;
pub use device::*;
pub use device_child::*;
// pub use dxgi::*;
pub use dsv::{
    DepthStencilDesc, DepthStencilOpDesc, DepthStencilState, DepthStencilView,
    DepthStencilViewDesc, DepthWriteMask, StencilOp,
};
pub use error::{Error, Result};
pub use input_layout::{InputClassification, InputElementDesc, InputLayout};
pub use query::{Counter, Predicate, Query};
pub use rasterizer::{CullMode, FillMode, RasterizerDesc, RasterizerState};
pub use resource::*;
pub use rtv::{RenderTargetView, RenderTargetViewDesc};
pub use sampler::{Filter, SamplerDesc, SamplerState, TextureAddress};
pub use shader::*;
pub use srv::{ShaderResourceView, ShaderResourceViewDesc};
pub use texture::{Texture1D, Texture1dDesc, Texture2D, Texture2dDesc, Texture3D, Texture3dDesc};
pub use uav::{UnorderedAccessView, UnorderedAccessViewDesc};

pub use d3d11_ffi as sys;
pub use d3d11_ffi::Foundation::HWND;
