pub mod format;
use bon::Builder;
pub use format::*;
pub mod swapchain;
pub use swapchain::*;

use d3d11_sys::Dxgi::Common::DXGI_SAMPLE_DESC;
use static_assertions::assert_eq_size;

#[repr(C)]
#[derive(Clone, Debug, Builder)]
pub struct SampleDesc {
    pub count: u32,
    pub quality: u32,
}
assert_eq_size!(SampleDesc, DXGI_SAMPLE_DESC);

impl Default for SampleDesc {
    fn default() -> Self {
        SampleDesc {
            count: 1,
            quality: 0,
        }
    }
}
