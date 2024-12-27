use std::mem::transmute;

use bon::Builder;
use static_assertions::assert_eq_size;
use windows::Win32::Graphics::Direct3D11::D3D11_VIEWPORT;

use crate::{util::OptionalParam, RasterizerState};

use super::DeviceContext;

impl DeviceContext {
    pub fn rasterizer_set_viewports(&self, viewports: &[Viewport]) {
        unsafe {
            self.0.RSSetViewports(Some(transmute(viewports)));
        }
    }

    pub fn rasterizer_set_state(&self, state: impl OptionalParam<Output = RasterizerState>) {
        unsafe {
            self.0.RSSetState(state.as_option().map(|s| &s.0));
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, Builder)]
pub struct Viewport {
    #[builder(default = 0.0)]
    pub top_left_x: f32,
    #[builder(default = 0.0)]
    pub top_left_y: f32,
    pub width: f32,
    pub height: f32,
    #[builder(default = 0.0)]
    pub min_depth: f32,
    #[builder(default = 1.0)]
    pub max_depth: f32,
}
assert_eq_size!(Viewport, D3D11_VIEWPORT);
