use std::mem::transmute;

use bon::Builder;
use windows::Win32::Graphics::Direct3D11::*;

use crate::{
    impl_device_child, impl_resource, verify_ffi_struct, BindFlags, CpuAccessFlags,
    ResourceMiscFlags, Usage,
};

#[repr(transparent)]
#[derive(Clone)]
pub struct Buffer(pub(crate) ID3D11Buffer);
impl_resource!(Buffer);

impl Buffer {
    pub fn get_desc(&self) -> BufferDesc {
        unsafe {
            let mut desc = D3D11_BUFFER_DESC::default();
            self.0.GetDesc(&mut desc);
            transmute(desc)
        }
    }
}

#[repr(C)]
#[derive(Builder, Clone, Debug)]
pub struct BufferDesc {
    pub byte_width: u32,
    #[builder(default)]
    pub usage: Usage,
    pub bind_flags: BindFlags,
    #[builder(default)]
    pub cpu_access_flags: CpuAccessFlags,
    #[builder(default)]
    pub misc_flags: ResourceMiscFlags,
    #[builder(default = 0)]
    pub structure_byte_stride: u32,
}
verify_ffi_struct!(BufferDesc, D3D11_BUFFER_DESC);
