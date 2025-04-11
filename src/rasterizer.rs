use bon::Builder;
use d3d11_sys::{Direct3D11::*, Foundation::BOOL};

use crate::{impl_device_child, verify_ffi_struct};

#[repr(transparent)]
#[derive(Clone)]
pub struct RasterizerState(pub(crate) ID3D11RasterizerState);
impl_device_child!(RasterizerState);

#[repr(C)]
#[derive(Debug, Builder, Clone)]
pub struct RasterizerDesc {
    pub fill_mode: FillMode,
    pub cull_mode: CullMode,
    #[builder(into)]
    pub front_counter_clockwise: BOOL,
    pub depth_bias: i32,
    pub depth_bias_clamp: f32,
    pub slope_scaled_depth_bias: f32,
    #[builder(into)]
    pub depth_clip_enable: BOOL,
    #[builder(into)]
    pub scissor_enable: BOOL,
    #[builder(into)]
    pub multisample_enable: BOOL,
    #[builder(into)]
    pub antialiased_line_enable: BOOL,
}
verify_ffi_struct!(RasterizerDesc, D3D11_RASTERIZER_DESC);

#[repr(i32)]
#[derive(Clone, Debug)]
pub enum FillMode {
    Wireframe = D3D11_FILL_WIREFRAME.0,
    Solid = D3D11_FILL_SOLID.0,
}

#[repr(i32)]
#[derive(Clone, Debug)]
pub enum CullMode {
    None = D3D11_CULL_NONE.0,
    Front = D3D11_CULL_FRONT.0,
    Back = D3D11_CULL_BACK.0,
}
