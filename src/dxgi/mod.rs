pub mod format;
pub use format::*;
pub mod swapchain;
pub use swapchain::*;

use static_assertions::assert_eq_size;
use windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC;

#[repr(C)]
#[derive(Clone, Debug)]
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
