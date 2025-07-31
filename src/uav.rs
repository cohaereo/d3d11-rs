use std::mem::transmute;

use bon::Builder;
use d3d11_ffi::Direct3D11::*;

use crate::{dxgi, impl_device_child, verify_ffi_struct};

#[repr(transparent)]
#[derive(Clone)]
pub struct UnorderedAccessView(pub(crate) ID3D11UnorderedAccessView);
impl_device_child!(UnorderedAccessView);

impl UnorderedAccessView {
    pub fn get_desc(&self) -> UnorderedAccessViewDesc {
        unsafe {
            let mut desc = D3D11_UNORDERED_ACCESS_VIEW_DESC::default();
            self.0.GetDesc(&mut desc);
            transmute(desc)
        }
    }
}

#[repr(C)]
#[derive(Builder, Clone, Debug)]
pub struct UnorderedAccessViewDesc {
    pub format: dxgi::Format,
    pub view_dimension: UavDimension,
}
verify_ffi_struct!(UnorderedAccessViewDesc, D3D11_UNORDERED_ACCESS_VIEW_DESC);

#[repr(i32)]
#[derive(Clone, Debug)]
pub enum UavDimension {
    Buffer {
        first_element: u32,
        num_elements: u32,
        flags: u32,
    } = D3D11_UAV_DIMENSION_BUFFER.0,
    Texture1D {
        mip_slice: u32,
    } = D3D11_UAV_DIMENSION_TEXTURE1D.0,
    Texture1DArray {
        mip_slice: u32,
        first_array_slice: u32,
        array_size: u32,
    } = D3D11_UAV_DIMENSION_TEXTURE1DARRAY.0,
    Texture2D {
        mip_slice: u32,
    } = D3D11_UAV_DIMENSION_TEXTURE2D.0,
    Texture2DArray {
        mip_slice: u32,
        first_array_slice: u32,
        array_size: u32,
    } = D3D11_UAV_DIMENSION_TEXTURE2DARRAY.0,
    Texture3D {
        mip_slice: u32,
    } = D3D11_UAV_DIMENSION_TEXTURE3D.0,
}

// bitflags! {
//     #[derive(Clone, Debug)]
//     pub struct SrvBufferExFlags: i32 {
//         const RAW = D3D11_BUFFEREX_SRV_FLAG_RAW.0;
//     }
// }
