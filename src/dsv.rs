use std::mem::transmute;

use bitflags::bitflags;
use bon::Builder;
use d3d11_sys::{Direct3D11::*, Foundation::BOOL};

use crate::{dxgi, impl_device_child, verify_ffi_struct, ComparisonFunc};

#[repr(transparent)]
#[derive(Clone)]
pub struct DepthStencilView(pub(crate) ID3D11DepthStencilView);
impl_device_child!(DepthStencilView);

impl DepthStencilView {
    pub fn get_desc(&self) -> DepthStencilViewDesc {
        unsafe {
            let mut desc = D3D11_DEPTH_STENCIL_VIEW_DESC::default();
            self.0.GetDesc(&mut desc);
            transmute(desc)
        }
    }
}

#[repr(C)]
#[derive(Builder, Clone)]
pub struct DepthStencilViewDesc {
    pub format: dxgi::Format,
    pub view_dimension: DsvDimension,
}
verify_ffi_struct!(DepthStencilViewDesc, D3D11_DEPTH_STENCIL_VIEW_DESC);

// TODO(cohae): The definition for D3D11_DEPTH_STENCIL_VIEW_DESC puts the flags in front of the union, but after the dimension field.
// We abuse the dimension field as tag for our enum, so we need to put the flags at the beginning of each variant.
#[repr(i32)]
#[derive(Clone)]
pub enum DsvDimension {
    Texture1D {
        flags: DsvFlags,
        mip_slice: u32,
    } = D3D11_DSV_DIMENSION_TEXTURE1D.0,
    Texture1DArray {
        flags: DsvFlags,
        mip_slice: u32,
        first_array_element: u32,
        array_size: u32,
    } = D3D11_DSV_DIMENSION_TEXTURE1DARRAY.0,
    Texture2D {
        flags: DsvFlags,
        mip_slice: u32,
    } = D3D11_DSV_DIMENSION_TEXTURE2D.0,
    Texture2DArray {
        flags: DsvFlags,
        mip_slice: u32,
        first_array_element: u32,
        array_size: u32,
    } = D3D11_DSV_DIMENSION_TEXTURE2DARRAY.0,
    Texture2DMS {
        flags: DsvFlags,
    } = D3D11_DSV_DIMENSION_TEXTURE2DMS.0,
    Texture2DMSArray {
        flags: DsvFlags,
        first_array_slice: u32,
        array_size: u32,
    } = D3D11_DSV_DIMENSION_TEXTURE2DMSARRAY.0,
}

bitflags! {
    #[derive(Clone)]
    pub struct DsvFlags: i32 {
        const READ_ONLY_DEPTH = D3D11_DSV_READ_ONLY_DEPTH.0;
        const READ_ONLY_STENCIL = D3D11_DSV_READ_ONLY_STENCIL.0;
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct DepthStencilState(pub(crate) ID3D11DepthStencilState);
impl_device_child!(DepthStencilState);

#[repr(C)]
#[derive(Builder, Clone, Debug)]
pub struct DepthStencilDesc {
    #[builder(into)]
    pub depth_enable: BOOL,
    pub depth_write_mask: DepthWriteMask,
    pub depth_func: ComparisonFunc,
    #[builder(into)]
    pub stencil_enable: BOOL,
    #[builder(default = 0xff)]
    pub stencil_read_mask: u8,
    #[builder(default = 0xff)]
    pub stencil_write_mask: u8,
    #[builder(default)]
    pub front_face: DepthStencilOpDesc,
    #[builder(default)]
    pub back_face: DepthStencilOpDesc,
}
verify_ffi_struct!(DepthStencilDesc, D3D11_DEPTH_STENCIL_DESC);

#[repr(C)]
#[derive(Builder, Clone, Debug)]
pub struct DepthStencilOpDesc {
    pub stencil_fail_op: StencilOp,
    pub stencil_depth_fail_op: StencilOp,
    pub stencil_pass_op: StencilOp,
    pub stencil_func: ComparisonFunc,
}
verify_ffi_struct!(DepthStencilOpDesc, D3D11_DEPTH_STENCILOP_DESC);

impl Default for DepthStencilOpDesc {
    fn default() -> Self {
        Self {
            stencil_fail_op: StencilOp::Keep,
            stencil_depth_fail_op: StencilOp::Keep,
            stencil_pass_op: StencilOp::Keep,
            stencil_func: ComparisonFunc::Always,
        }
    }
}

#[repr(i32)]
#[derive(Clone, Debug)]
pub enum StencilOp {
    Keep = D3D11_STENCIL_OP_KEEP.0,
    Zero = D3D11_STENCIL_OP_ZERO.0,
    Replace = D3D11_STENCIL_OP_REPLACE.0,
    IncrementSat = D3D11_STENCIL_OP_INCR_SAT.0,
    DecrementSat = D3D11_STENCIL_OP_DECR_SAT.0,
    Invert = D3D11_STENCIL_OP_INVERT.0,
    Increment = D3D11_STENCIL_OP_INCR.0,
    Decrement = D3D11_STENCIL_OP_DECR.0,
}

#[repr(i32)]
#[derive(Clone, Debug)]
pub enum DepthWriteMask {
    Zero = D3D11_DEPTH_WRITE_MASK_ZERO.0,
    All = D3D11_DEPTH_WRITE_MASK_ALL.0,
}
