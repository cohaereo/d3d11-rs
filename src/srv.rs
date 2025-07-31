use std::mem::transmute;

use bitflags::bitflags;
use bon::Builder;
use d3d11_ffi::{Direct3D::*, Direct3D11::*};

use crate::{dxgi, impl_device_child, verify_ffi_struct};

#[repr(transparent)]
#[derive(Clone)]
pub struct ShaderResourceView(pub(crate) ID3D11ShaderResourceView);
impl_device_child!(ShaderResourceView);

impl ShaderResourceView {
    pub fn get_desc(&self) -> ShaderResourceViewDesc {
        unsafe {
            let mut desc = D3D11_SHADER_RESOURCE_VIEW_DESC::default();
            self.0.GetDesc(&mut desc);
            transmute(desc)
        }
    }
}

#[repr(C)]
#[derive(Builder, Clone, Debug)]
pub struct ShaderResourceViewDesc {
    pub format: dxgi::Format,
    pub view_dimension: SrvDimension,
}
verify_ffi_struct!(ShaderResourceViewDesc, D3D11_SHADER_RESOURCE_VIEW_DESC);

#[repr(i32)]
#[derive(Clone, Debug)]
pub enum SrvDimension {
    Buffer {
        first_element_or_element_offset: u32,
        num_elements_or_element_width: u32,
    } = D3D11_SRV_DIMENSION_BUFFER.0,
    Texture1D {
        most_detailed_mip: u32,
        mip_levels: u32,
    } = D3D11_SRV_DIMENSION_TEXTURE1D.0,
    Texture1DArray {
        most_detailed_mip: u32,
        mip_levels: u32,
        first_array_slice: u32,
        array_size: u32,
    } = D3D11_SRV_DIMENSION_TEXTURE1DARRAY.0,
    Texture2D {
        most_detailed_mip: u32,
        mip_levels: u32,
    } = D3D11_SRV_DIMENSION_TEXTURE2D.0,
    Texture2DArray {
        most_detailed_mip: u32,
        mip_levels: u32,
        first_array_slice: u32,
        array_size: u32,
    } = D3D11_SRV_DIMENSION_TEXTURE2DARRAY.0,
    Texture2DMS = D3D11_SRV_DIMENSION_TEXTURE2DMS.0,
    Texture2DMSArray {
        first_array_slice: u32,
        array_size: u32,
    } = D3D11_SRV_DIMENSION_TEXTURE2DMSARRAY.0,
    Texture3D {
        most_detailed_mip: u32,
        mip_levels: u32,
    } = D3D11_SRV_DIMENSION_TEXTURE3D.0,
    TextureCube {
        most_detailed_mip: u32,
        mip_levels: u32,
    } = D3D11_SRV_DIMENSION_TEXTURECUBE.0,
    TextureCubeArray {
        most_detailed_mip: u32,
        mip_levels: u32,
        first_2d_array_face: u32,
        num_cubes: u32,
    } = D3D11_SRV_DIMENSION_TEXTURECUBEARRAY.0,
    BufferEx {
        first_element: u32,
        num_elements: u32,
        flags: SrvBufferExFlags,
    } = D3D11_SRV_DIMENSION_BUFFEREX.0,
}

impl SrvDimension {
    pub fn is_texture(&self) -> bool {
        matches!(
            self,
            Self::Texture1D { .. }
                | Self::Texture1DArray { .. }
                | Self::Texture2D { .. }
                | Self::Texture2DArray { .. }
                | Self::Texture2DMS { .. }
                | Self::Texture2DMSArray { .. }
                | Self::Texture3D { .. }
                | Self::TextureCube { .. }
                | Self::TextureCubeArray { .. }
        )
    }
}

bitflags! {
    #[derive(Clone, Debug)]
    pub struct SrvBufferExFlags: i32 {
        const RAW = D3D11_BUFFEREX_SRV_FLAG_RAW.0;
    }
}
