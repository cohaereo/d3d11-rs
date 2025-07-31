use std::mem::transmute;

use bon::Builder;
use d3d11_ffi::Direct3D11::*;

use crate::{
    dxgi, impl_device_child, impl_resource,
    resource::{BindFlags, CpuAccessFlags, ResourceMiscFlags, Usage},
    verify_ffi_struct,
};

#[repr(transparent)]
#[derive(Clone)]
pub struct Texture1D(pub(crate) ID3D11Texture1D);
impl_resource!(Texture1D);

impl Texture1D {
    pub fn get_desc(&self) -> Texture1dDesc {
        unsafe {
            let mut desc = D3D11_TEXTURE1D_DESC::default();
            self.0.GetDesc(&mut desc);
            transmute(desc)
        }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct Texture2D(pub(crate) ID3D11Texture2D);
impl_resource!(Texture2D);

impl Texture2D {
    pub fn get_desc(&self) -> Texture2dDesc {
        unsafe {
            let mut desc = D3D11_TEXTURE2D_DESC::default();
            self.0.GetDesc(&mut desc);
            transmute(desc)
        }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct Texture3D(pub(crate) ID3D11Texture3D);
impl_resource!(Texture3D);

impl Texture3D {
    pub fn get_desc(&self) -> Texture3dDesc {
        unsafe {
            let mut desc = D3D11_TEXTURE3D_DESC::default();
            self.0.GetDesc(&mut desc);
            transmute(desc)
        }
    }
}

#[repr(C)]
#[derive(Builder, Clone, Debug)]
pub struct Texture1dDesc {
    pub width: u32,
    pub mip_levels: u32,
    pub array_size: u32,
    pub format: dxgi::Format,
    #[builder(default)]
    pub usage: Usage,
    pub bind_flags: BindFlags,
    #[builder(default)]
    pub cpu_access_flags: CpuAccessFlags,
    #[builder(default)]
    pub misc_flags: ResourceMiscFlags,
}
verify_ffi_struct!(Texture1dDesc, D3D11_TEXTURE1D_DESC);

#[repr(C)]
#[derive(Builder, Clone, Debug)]
pub struct Texture2dDesc {
    pub width: u32,
    pub height: u32,
    pub mip_levels: u32,
    #[builder(default = 1)]
    pub array_size: u32,
    pub format: dxgi::Format,
    #[builder(default)]
    pub sample_desc: dxgi::SampleDesc,
    #[builder(default)]
    pub usage: Usage,
    pub bind_flags: BindFlags,
    #[builder(default)]
    pub cpu_access_flags: CpuAccessFlags,
    #[builder(default)]
    pub misc_flags: ResourceMiscFlags,
}
verify_ffi_struct!(Texture2dDesc, D3D11_TEXTURE2D_DESC);

impl Texture2dDesc {
    pub fn resolution(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

#[repr(C)]
#[derive(Builder, Clone, Debug)]
pub struct Texture3dDesc {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mip_levels: u32,
    pub format: dxgi::Format,
    #[builder(default)]
    pub usage: Usage,
    pub bind_flags: BindFlags,
    #[builder(default)]
    pub cpu_access_flags: CpuAccessFlags,
    #[builder(default)]
    pub misc_flags: ResourceMiscFlags,
}
verify_ffi_struct!(Texture3dDesc, D3D11_TEXTURE3D_DESC);
