use bon::Builder;
use d3d11_sys::Direct3D11::*;

use crate::{common::ComparisonFunc, verify_ffi_struct};

#[repr(transparent)]
#[derive(Clone)]
pub struct SamplerState(pub(crate) ID3D11SamplerState);

#[repr(C)]
#[derive(Builder)]
pub struct SamplerDesc {
    #[builder(default = Filter::MinMagMipLinear)]
    pub filter: Filter,
    #[builder(default = TextureAddress::Clamp)]
    pub address_u: TextureAddress,
    #[builder(default = TextureAddress::Clamp)]
    pub address_v: TextureAddress,
    #[builder(default = TextureAddress::Clamp)]
    pub address_w: TextureAddress,
    #[builder(default = 0.0)]
    pub mip_lod_bias: f32,
    #[builder(default = 1)]
    pub max_anisotropy: u32,
    #[builder(default = ComparisonFunc::Always)]
    pub comparison_func: ComparisonFunc,
    #[builder(default = [0.0, 0.0, 0.0, 0.0])]
    pub border_color: [f32; 4],
    #[builder(default = 0.0)]
    pub min_lod: f32,
    #[builder(default = f32::MAX)]
    pub max_lod: f32,
}
verify_ffi_struct!(SamplerDesc, D3D11_SAMPLER_DESC);

impl Default for SamplerDesc {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[repr(i32)]
#[rustfmt::skip]
pub enum Filter {
    Anisotropic = D3D11_FILTER_ANISOTROPIC.0,
    ComparisonAnisotropic = D3D11_FILTER_COMPARISON_ANISOTROPIC.0,
    ComparisonMinLinearMagMipPoint = D3D11_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT.0,
    ComparisonMinLinearMagPointMipLinear = D3D11_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR.0,
    ComparisonMinMagLinearMipPoint = D3D11_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT.0,
    ComparisonMinMagMipLinear = D3D11_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR.0,
    ComparisonMinMagMipPoint = D3D11_FILTER_COMPARISON_MIN_MAG_MIP_POINT.0,
    ComparisonMinMagPointMipLinear = D3D11_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR.0,
    ComparisonMinPointMagLinearMipPoint = D3D11_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT.0,
    ComparisonMinPointMagMipLinear = D3D11_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR.0,
    MaximumAnisotropic = D3D11_FILTER_MAXIMUM_ANISOTROPIC.0,
    MaximumMinLinearMagMipPoint = D3D11_FILTER_MAXIMUM_MIN_LINEAR_MAG_MIP_POINT.0,
    MaximumMinLinearMagPointMipLinear = D3D11_FILTER_MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR.0,
    MaximumMinMagLinearMipPoint = D3D11_FILTER_MAXIMUM_MIN_MAG_LINEAR_MIP_POINT.0,
    MaximumMinMagMipLinear = D3D11_FILTER_MAXIMUM_MIN_MAG_MIP_LINEAR.0,
    MaximumMinMagMipPoint = D3D11_FILTER_MAXIMUM_MIN_MAG_MIP_POINT.0,
    MaximumMinMagPointMipLinear = D3D11_FILTER_MAXIMUM_MIN_MAG_POINT_MIP_LINEAR.0,
    MaximumMinPointMagLinearMipPoint = D3D11_FILTER_MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT.0,
    MaximumMinPointMagMipLinear = D3D11_FILTER_MAXIMUM_MIN_POINT_MAG_MIP_LINEAR.0,
    MinimumAnisotropic = D3D11_FILTER_MINIMUM_ANISOTROPIC.0,
    MinimumMinLinearMagMipPoint = D3D11_FILTER_MINIMUM_MIN_LINEAR_MAG_MIP_POINT.0,
    MinimumMinLinearMagPointMipLinear = D3D11_FILTER_MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR.0,
    MinimumMinMagLinearMipPoint = D3D11_FILTER_MINIMUM_MIN_MAG_LINEAR_MIP_POINT.0,
    MinimumMinMagMipLinear = D3D11_FILTER_MINIMUM_MIN_MAG_MIP_LINEAR.0,
    MinimumMinMagMipPoint = D3D11_FILTER_MINIMUM_MIN_MAG_MIP_POINT.0,
    MinimumMinMagPointMipLinear = D3D11_FILTER_MINIMUM_MIN_MAG_POINT_MIP_LINEAR.0,
    MinimumMinPointMagLinearMipPoint = D3D11_FILTER_MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT.0,
    MinimumMinPointMagMipLinear = D3D11_FILTER_MINIMUM_MIN_POINT_MAG_MIP_LINEAR.0,
    MinLinearMagMipPoint = D3D11_FILTER_MIN_LINEAR_MAG_MIP_POINT.0,
    MinLinearMagPointMipLinear = D3D11_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR.0,
    MinMagLinearMipPoint = D3D11_FILTER_MIN_MAG_LINEAR_MIP_POINT.0,
    MinMagMipLinear = D3D11_FILTER_MIN_MAG_MIP_LINEAR.0,
    MinMagMipPoint = D3D11_FILTER_MIN_MAG_MIP_POINT.0,
    MinMagPointMipLinear = D3D11_FILTER_MIN_MAG_POINT_MIP_LINEAR.0,
    MinPointMagLinearMipPoint = D3D11_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT.0,
    MinPointMagMipLinear = D3D11_FILTER_MIN_POINT_MAG_MIP_LINEAR.0,
}

#[repr(i32)]
#[rustfmt::skip]
pub enum TextureAddress {
    Border = D3D11_TEXTURE_ADDRESS_BORDER.0,
    Clamp = D3D11_TEXTURE_ADDRESS_CLAMP.0,
    Mirror = D3D11_TEXTURE_ADDRESS_MIRROR.0,
    MirrorOnce = D3D11_TEXTURE_ADDRESS_MIRROR_ONCE.0,
    Wrap = D3D11_TEXTURE_ADDRESS_WRAP.0,
}
