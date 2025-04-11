use std::mem::transmute;

use bon::Builder;
use d3d11_sys::Direct3D11::D3D11_VIEWPORT;
use static_assertions::assert_eq_size;

use crate::{util::OptionalParam, RasterizerState};

use super::DeviceContext;

impl DeviceContext {
    pub fn rasterizer_set_viewports(&self, viewports: &[Viewport]) {
        unsafe {
            self.0.RSSetViewports(Some(transmute(viewports)));
        }
    }

    pub fn rasterizer_get_viewports(&self) -> [Viewport; 8] {
        let mut viewports: [D3D11_VIEWPORT; 8] = std::array::from_fn(|_| Default::default());
        unsafe {
            self.0.RSGetViewports(&mut 8, Some(viewports.as_mut_ptr()));
            viewports.map(|v| transmute(v))
        }
    }

    pub fn rasterizer_set_scissor_rects(&self, rects: &[Rect]) {
        unsafe {
            self.0.RSSetScissorRects(Some(transmute(rects)));
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

impl Default for Viewport {
    fn default() -> Self {
        Self {
            top_left_x: 0.0,
            top_left_y: 0.0,
            width: 0.0,
            height: 0.0,
            min_depth: 0.0,
            max_depth: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, Builder)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
assert_eq_size!(Rect, d3d11_sys::Foundation::RECT);
