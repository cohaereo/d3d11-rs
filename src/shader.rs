use windows::{
    core::PCSTR,
    Win32::Graphics::{
        Direct3D::{Fxc::D3DCompile, D3D_SHADER_MACRO},
        Direct3D11::*,
    },
};

use crate::{error::bail, impl_device_child, util::to_pcstr};

#[repr(transparent)]
pub struct VertexShader(pub(crate) ID3D11VertexShader);
impl_device_child!(VertexShader);

#[repr(transparent)]
pub struct PixelShader(pub(crate) ID3D11PixelShader);
impl_device_child!(PixelShader);

#[repr(transparent)]
pub struct GeometryShader(pub(crate) ID3D11GeometryShader);
impl_device_child!(GeometryShader);

#[repr(transparent)]
pub struct HullShader(pub(crate) ID3D11HullShader);
impl_device_child!(HullShader);

#[repr(transparent)]
pub struct DomainShader(pub(crate) ID3D11DomainShader);
impl_device_child!(DomainShader);

#[repr(transparent)]
pub struct ComputeShader(pub(crate) ID3D11ComputeShader);
impl_device_child!(ComputeShader);

/// Provides a safe wrapper around the D3DCompile function.
pub fn compile(
    data: &[u8],
    source_name: Option<&str>,
    defines: &[(&str, &str)],
    // include: ???, // TODO
    entry_point: &str,
    target: ShaderTarget,
) -> crate::Result<Vec<u8>> {
    let mut pcode = None;
    let mut perrormsgs = None;

    let source_name = source_name.map(to_pcstr);
    let (_centrypoint, pentrypoint) = to_pcstr(entry_point);
    let (_ctarget, ptarget) = to_pcstr(target.profile());

    let mut defines_processed = vec![];
    // Holds allocated strings for the lifetime of the function, making sure they're deallocated when the function returns
    let mut define_strdata = vec![];
    for (name, value) in defines {
        let (name, name_ptr) = to_pcstr(name);
        let (value, value_ptr) = to_pcstr(value);
        define_strdata.push(name);
        define_strdata.push(value);
        defines_processed.push(D3D_SHADER_MACRO {
            Name: name_ptr,
            Definition: value_ptr,
        });
    }
    // Last entry must be null, acting as a terminator
    defines_processed.push(D3D_SHADER_MACRO {
        Name: PCSTR::null(),
        Definition: PCSTR::null(),
    });

    unsafe {
        match D3DCompile(
            data.as_ptr() as *const _,
            data.len() as _,
            source_name.map(|s| s.1).unwrap_or(PCSTR::null()),
            Some(defines_processed.as_ptr()),
            None,
            pentrypoint,
            ptarget,
            0, // TODO
            0, // Effect-related flags, not relevant
            &raw mut pcode,
            Some(&raw mut perrormsgs),
        ) {
            Ok(_) => {}
            Err(e) => {
                let perrormsgs = perrormsgs.unwrap();
                let errormsgs = std::slice::from_raw_parts(
                    perrormsgs.GetBufferPointer() as *const u8,
                    perrormsgs.GetBufferSize() as _,
                );
                let errormsgs = std::str::from_utf8(errormsgs).unwrap();
                bail!("Failed to compile shader: {e}.\nCompiler messages:\n{errormsgs}");
            }
        }
    }

    let data = unsafe {
        let pcode = pcode.unwrap();
        std::slice::from_raw_parts(
            pcode.GetBufferPointer() as *const u8,
            pcode.GetBufferSize() as _,
        )
        .to_vec()
    };

    Ok(data)
}

pub enum ShaderTarget {
    Vertex,
    Pixel,
    Geometry,
    Hull,
    Domain,
    Compute,
}

impl ShaderTarget {
    pub fn profile(&self) -> &'static str {
        match self {
            Self::Vertex => "vs_5_0",
            Self::Pixel => "ps_5_0",
            Self::Geometry => "gs_5_0",
            Self::Hull => "hs_5_0",
            Self::Domain => "ds_5_0",
            Self::Compute => "cs_5_0",
        }
    }
}
