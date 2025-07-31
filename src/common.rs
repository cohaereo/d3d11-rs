use d3d11_ffi::Direct3D11::*;

use crate::verify_ffi_struct;

#[repr(i32)]
#[derive(Clone, Debug)]
pub enum ComparisonFunc {
    Always = D3D11_COMPARISON_ALWAYS.0,
    Equal = D3D11_COMPARISON_EQUAL.0,
    Greater = D3D11_COMPARISON_GREATER.0,
    GreaterEqual = D3D11_COMPARISON_GREATER_EQUAL.0,
    Less = D3D11_COMPARISON_LESS.0,
    LessEqual = D3D11_COMPARISON_LESS_EQUAL.0,
    Never = D3D11_COMPARISON_NEVER.0,
    NotEqual = D3D11_COMPARISON_NOT_EQUAL.0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Box3D {
    pub left: i32,
    pub top: i32,
    pub front: i32,
    pub right: i32,
    pub bottom: i32,
    pub back: i32,
}

verify_ffi_struct!(Box3D, D3D11_BOX);

pub fn calc_subresource_index(mip_slice: u32, array_slice: u32, mip_levels: u32) -> u32 {
    mip_slice + array_slice * mip_levels
}

pub fn calc_mip_count(width: u32, height: u32) -> u32 {
    let mut count = 1;
    let mut w = width;
    let mut h = height;

    while w > 1 || h > 1 {
        count += 1;
        w = w.div_ceil(2);
        h = h.div_ceil(2);
    }

    count
}
