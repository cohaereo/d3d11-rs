use d3d11_ffi::{
    core_supplemental::PCSTR,
    Direct3D::{
        Fxc::{D3DCompile, D3DCOMPILE_DEBUG},
        D3D_SHADER_MACRO,
    },
    Direct3D11::*,
};

use crate::{error::bail, fxc::ShaderTarget, impl_device_child, util::to_pcstr};

#[repr(transparent)]
#[derive(Clone)]
pub struct VertexShader(pub(crate) ID3D11VertexShader);
impl_device_child!(VertexShader);

#[repr(transparent)]
#[derive(Clone)]
pub struct PixelShader(pub(crate) ID3D11PixelShader);
impl_device_child!(PixelShader);

#[repr(transparent)]
#[derive(Clone)]
pub struct GeometryShader(pub(crate) ID3D11GeometryShader);
impl_device_child!(GeometryShader);

#[repr(transparent)]
#[derive(Clone)]
pub struct HullShader(pub(crate) ID3D11HullShader);
impl_device_child!(HullShader);

#[repr(transparent)]
#[derive(Clone)]
pub struct DomainShader(pub(crate) ID3D11DomainShader);
impl_device_child!(DomainShader);

#[repr(transparent)]
#[derive(Clone)]
pub struct ComputeShader(pub(crate) ID3D11ComputeShader);
impl_device_child!(ComputeShader);

#[cfg(feature = "fxc")]
#[deprecated(since = "0.3.0", note = "Use d3d11::fxc::compile instead")]
pub fn fxc_compile(
    data: &[u8],
    source_name: Option<&str>,
    defines: &[(&str, &str)],
    // include: ???, // TODO
    entry_point: &str,
    target: ShaderTarget,
) -> crate::Result<Vec<u8>> {
    crate::fxc::compile(
        data,
        source_name,
        defines,
        // include,
        entry_point,
        target,
    )
}
