use bitflags::bitflags;
use d3d11_ffi::{Direct3D::*, Direct3D11::*, Dxgi::IDXGIAdapter, Foundation::HMODULE};

use crate::{
    blend_state::{BlendDesc, BlendState},
    context::DeviceContext,
    dsv::{DepthStencilDesc, DepthStencilState},
    dxgi,
    error::validate_input,
    input_layout::{ElementOffset, InputElementDesc, InputLayout},
    query::QueryDesc,
    rasterizer::{RasterizerDesc, RasterizerState},
    rtv::{RenderTargetView, RenderTargetViewDesc},
    sampler::{SamplerDesc, SamplerState},
    shader::*,
    srv::{ShaderResourceView, ShaderResourceViewDesc},
    texture::{Texture1D, Texture1dDesc, Texture2D, Texture2dDesc, Texture3D, Texture3dDesc},
    uav::{UnorderedAccessView, UnorderedAccessViewDesc},
    util::{to_pcstr, wrap_option_out_result, OptionalParam},
    Buffer, BufferDesc, DepthStencilView, DepthStencilViewDesc, Query, Resource,
};

// TODO(cohae): How do we handle newer versions of the API? eg. ID3D11Device1
#[repr(transparent)]
#[derive(Clone)]
pub struct Device(pub(crate) ID3D11Device);

// Initialization
impl Device {
    /// Temporary method for creating a device.
    /// ImmediateContext can be retrieved by calling `device.immediate_context()`
    // TODO:  Abstract away IDXGIAdapter -> dxgi::Adapter
    pub fn create(adapter: Option<IDXGIAdapter>, debug: bool) -> crate::Result<Self> {
        let mut device = None;
        let driver_type = if adapter.is_some() {
            D3D_DRIVER_TYPE_UNKNOWN
        } else {
            D3D_DRIVER_TYPE_HARDWARE
        };

        unsafe {
            D3D11CreateDevice(
                adapter.as_ref(),
                driver_type,
                HMODULE::default(),
                if debug {
                    D3D11_CREATE_DEVICE_DEBUG
                } else {
                    Default::default()
                },
                Some(&[D3D_FEATURE_LEVEL_11_1, D3D_FEATURE_LEVEL_11_0]),
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                None,
            )?;
        }

        Ok(Self(device.unwrap()))
    }
}

macro_rules! generate_create_shader_method {
    ($name:ident, $method:ident, $shader_type:ident) => {
        pub fn $name(&self, bytecode: &[u8]) -> crate::Result<$shader_type> {
            let inner =
                wrap_option_out_result(|out| unsafe { self.0.$method(bytecode, None, out) })?;

            Ok($shader_type(inner))
        }
    };
}

#[cfg_attr(feature = "profiling", profiling::all_functions)]
impl Device {
    pub fn get_immediate_context(&self) -> DeviceContext {
        DeviceContext::from_raw(
            unsafe { self.0.GetImmediateContext() }.expect("Failed to get immediate context"),
        )
    }

    pub fn create_deferred_context(&self) -> crate::Result<DeviceContext> {
        let inner = wrap_option_out_result(|out| unsafe { self.0.CreateDeferredContext(0, out) })?;

        Ok(DeviceContext::from_raw(inner))
    }

    pub fn create_sampler_state(&self, desc: &SamplerDesc) -> crate::Result<SamplerState> {
        let inner =
            wrap_option_out_result(|out| unsafe { self.0.CreateSamplerState(desc.as_ffi(), out) })?;

        Ok(SamplerState(inner))
    }

    pub fn create_buffer(
        &self,
        desc: &BufferDesc,
        initial_data: Option<&[u8]>,
    ) -> crate::Result<Buffer> {
        validate_input!(
            !(initial_data.is_none() & desc.usage.is_immutable()),
            "Initial data must be provided for immutable buffers"
        );
        validate_input!(desc.byte_width > 0, "Buffer size must be greater than zero");

        let initial_data = initial_data.map(|d| D3D11_SUBRESOURCE_DATA {
            pSysMem: d.as_ptr() as _,
            SysMemPitch: d.len() as u32,
            SysMemSlicePitch: 0,
        });

        let inner = wrap_option_out_result(|out| unsafe {
            self.0
                .CreateBuffer(desc.as_ffi(), initial_data.as_ref().map(|d| d as _), out)
        })?;

        Ok(Buffer(inner))
    }

    fn validate_texture_desc(
        format: dxgi::Format,
        mips_given: usize,
        mip_levels: u32,
        array_size: u32,
    ) -> crate::Result<()> {
        validate_input!(
            mips_given as u32 == (array_size * mip_levels),
            "Invalid number of initial data structures provided (expected {}, got {})",
            array_size * mip_levels,
            mips_given
        );

        validate_input!(
            !(mips_given == 0 && format.is_compressed()),
            "Cannot generate mipmaps for compressed formats (zero mip levels given)"
        );

        Ok(())
    }

    pub fn create_texture1d(
        &self,
        desc: &Texture1dDesc,
        // TODO: Provide a safer, more ergonomic way to pass initial data
        initial_data: Option<&[D3D11_SUBRESOURCE_DATA]>,
    ) -> crate::Result<Texture1D> {
        if let Some(id) = initial_data {
            Self::validate_texture_desc(desc.format, id.len(), desc.mip_levels, 1)?;
        } else {
            validate_input!(
                !desc.usage.is_immutable(),
                "Initial data must be provided for immutable textures"
            );
        }

        let inner = wrap_option_out_result(|out| unsafe {
            self.0
                .CreateTexture1D(desc.as_ffi(), initial_data.map(|d| d.as_ptr()), out)
        })?;

        Ok(Texture1D(inner))
    }

    pub fn create_texture2d(
        &self,
        desc: &Texture2dDesc,
        // TODO: Provide a safer, more ergonomic way to pass initial data
        initial_data: Option<&[D3D11_SUBRESOURCE_DATA]>,
    ) -> crate::Result<Texture2D> {
        if let Some(id) = initial_data {
            Self::validate_texture_desc(desc.format, id.len(), desc.mip_levels, desc.array_size)?;
        } else {
            validate_input!(
                !desc.usage.is_immutable(),
                "Initial data must be provided for immutable textures"
            );
        }

        let inner = wrap_option_out_result(|out| unsafe {
            self.0
                .CreateTexture2D(desc.as_ffi(), initial_data.map(|d| d.as_ptr()), out)
        })?;

        Ok(Texture2D(inner))
    }

    pub fn create_texture3d(
        &self,
        desc: &Texture3dDesc,
        // TODO: Provide a safer, more ergonomic way to pass initial data
        initial_data: Option<&[D3D11_SUBRESOURCE_DATA]>,
    ) -> crate::Result<Texture3D> {
        if let Some(id) = initial_data {
            Self::validate_texture_desc(desc.format, id.len(), desc.mip_levels, 1)?;
        } else {
            validate_input!(
                !desc.usage.is_immutable(),
                "Initial data must be provided for immutable textures"
            );
        }

        let inner = wrap_option_out_result(|out| unsafe {
            self.0
                .CreateTexture3D(desc.as_ffi(), initial_data.map(|d| d.as_ptr()), out)
        })?;

        Ok(Texture3D(inner))
    }

    generate_create_shader_method!(create_vertex_shader, CreateVertexShader, VertexShader);
    generate_create_shader_method!(create_pixel_shader, CreatePixelShader, PixelShader);
    generate_create_shader_method!(create_geometry_shader, CreateGeometryShader, GeometryShader);
    generate_create_shader_method!(create_hull_shader, CreateHullShader, HullShader);
    generate_create_shader_method!(create_domain_shader, CreateDomainShader, DomainShader);
    generate_create_shader_method!(create_compute_shader, CreateComputeShader, ComputeShader);

    /// Creates a RenderTargetView for the given resource
    ///
    /// Set `desc` to None to create a view that accesses all of the subresources in mipmap level 0
    pub fn create_render_target_view(
        &self,
        resource: &impl Resource,
        desc: Option<&RenderTargetViewDesc>,
    ) -> crate::Result<RenderTargetView> {
        if let Some(desc) = desc {
            validate_input!(!desc.format.is_typeless(), "Format must not be typeless");
        }

        let inner = wrap_option_out_result(|out| unsafe {
            self.0
                .CreateRenderTargetView(resource.as_ffi_resource(), desc.map(|d| d.as_ffi()), out)
        })?;

        Ok(RenderTargetView(inner))
    }

    pub fn create_depth_stencil_view(
        &self,
        resource: &impl Resource,
        desc: Option<&DepthStencilViewDesc>,
    ) -> crate::Result<DepthStencilView> {
        let inner = wrap_option_out_result(|out| unsafe {
            self.0
                .CreateDepthStencilView(resource.as_ffi_resource(), desc.map(|d| d.as_ffi()), out)
        })?;

        Ok(DepthStencilView(inner))
    }

    pub fn create_shader_resource_view(
        &self,
        resource: &impl Resource,
        desc: impl OptionalParam<Output = ShaderResourceViewDesc>,
    ) -> crate::Result<ShaderResourceView> {
        if let Some(desc) = desc.as_option() {
            if desc.view_dimension.is_texture() {
                validate_input!(
                    !desc.format.is_typeless(),
                    "SRV format must not be typeless"
                );
            }
        }

        let inner = wrap_option_out_result(|out| unsafe {
            self.0.CreateShaderResourceView(
                resource.as_ffi_resource(),
                desc.as_option().map(|d| d.as_ffi()),
                out,
            )
        })?;

        Ok(ShaderResourceView(inner))
    }

    pub fn create_unordered_access_view(
        &self,
        resource: &impl Resource,
        desc: impl OptionalParam<Output = UnorderedAccessViewDesc>,
    ) -> crate::Result<UnorderedAccessView> {
        let inner = wrap_option_out_result(|out| unsafe {
            self.0.CreateUnorderedAccessView(
                resource.as_ffi_resource(),
                desc.as_option().map(|d| d.as_ffi()),
                out,
            )
        })?;

        Ok(UnorderedAccessView(inner))
    }

    pub fn create_input_layout(
        &self,
        desc: &[InputElementDesc],
        shader_bytecode: &[u8],
    ) -> crate::Result<InputLayout> {
        let mut strings = vec![];
        let mut descs_ffi = vec![];
        for d in desc {
            let (cstring, pstring) = to_pcstr(&d.semantic_name);
            strings.push(cstring);

            descs_ffi.push(D3D11_INPUT_ELEMENT_DESC {
                SemanticName: pstring,
                SemanticIndex: d.semantic_index,
                Format: d.format.into(),
                InputSlot: d.input_slot,
                AlignedByteOffset: match d.aligned_byte_offset {
                    ElementOffset::Append => D3D11_APPEND_ALIGNED_ELEMENT,
                    ElementOffset::Absolute(offset) => offset,
                },
                InputSlotClass: d.input_slot_class.clone().into(),
                InstanceDataStepRate: d.instance_data_step_rate,
            });
        }

        let inner = wrap_option_out_result(|out| unsafe {
            self.0.CreateInputLayout(&descs_ffi, shader_bytecode, out)
        })?;

        Ok(InputLayout(inner))
    }

    pub fn create_rasterizer_state(&self, desc: &RasterizerDesc) -> crate::Result<RasterizerState> {
        let inner = wrap_option_out_result(|out| unsafe {
            self.0.CreateRasterizerState(desc.as_ffi(), out)
        })?;

        Ok(RasterizerState(inner))
    }

    pub fn create_depth_stencil_state(
        &self,
        desc: &DepthStencilDesc,
    ) -> crate::Result<DepthStencilState> {
        let inner = wrap_option_out_result(|out| unsafe {
            self.0.CreateDepthStencilState(desc.as_ffi(), out)
        })?;

        Ok(DepthStencilState(inner))
    }

    pub fn create_blend_state(&self, desc: &BlendDesc) -> crate::Result<BlendState> {
        let inner =
            wrap_option_out_result(|out| unsafe { self.0.CreateBlendState(desc.as_ffi(), out) })?;

        Ok(BlendState(inner))
    }

    pub fn create_query(&self, desc: &QueryDesc) -> crate::Result<Query> {
        let inner =
            wrap_option_out_result(|out| unsafe { self.0.CreateQuery(desc.as_ffi(), out) })?;

        Ok(Query(inner))
    }

    pub fn check_format_support(&self, format: dxgi::Format) -> FormatSupport {
        FormatSupport::from_bits_truncate(unsafe {
            self.0.CheckFormatSupport(format.into()).unwrap()
        })
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct FormatSupport : u32 {
        const BACK_BUFFER_CAST = D3D11_FORMAT_SUPPORT_BACK_BUFFER_CAST.0 as u32;
        const BLENDABLE = D3D11_FORMAT_SUPPORT_BLENDABLE.0 as u32;
        const BUFFER = D3D11_FORMAT_SUPPORT_BUFFER.0 as u32;
        const CAST_WITHIN_BIT_LAYOUT = D3D11_FORMAT_SUPPORT_CAST_WITHIN_BIT_LAYOUT.0 as u32;
        const CPU_LOCKABLE = D3D11_FORMAT_SUPPORT_CPU_LOCKABLE.0 as u32;
        const DECODER_OUTPUT = D3D11_FORMAT_SUPPORT_DECODER_OUTPUT.0 as u32;
        const DEPTH_STENCIL = D3D11_FORMAT_SUPPORT_DEPTH_STENCIL.0 as u32;
        const DISPLAY = D3D11_FORMAT_SUPPORT_DISPLAY.0 as u32;
        const IA_INDEX_BUFFER = D3D11_FORMAT_SUPPORT_IA_INDEX_BUFFER.0 as u32;
        const IA_VERTEX_BUFFER = D3D11_FORMAT_SUPPORT_IA_VERTEX_BUFFER.0 as u32;
        const MIP = D3D11_FORMAT_SUPPORT_MIP.0 as u32;
        const MIP_AUTOGEN = D3D11_FORMAT_SUPPORT_MIP_AUTOGEN.0 as u32;
        const MULTISAMPLE_LOAD = D3D11_FORMAT_SUPPORT_MULTISAMPLE_LOAD.0 as u32;
        const MULTISAMPLE_RENDERTARGET = D3D11_FORMAT_SUPPORT_MULTISAMPLE_RENDERTARGET.0 as u32;
        const MULTISAMPLE_RESOLVE = D3D11_FORMAT_SUPPORT_MULTISAMPLE_RESOLVE.0 as u32;
        const RENDER_TARGET = D3D11_FORMAT_SUPPORT_RENDER_TARGET.0 as u32;
        const SHADER_GATHER = D3D11_FORMAT_SUPPORT_SHADER_GATHER.0 as u32;
        const SHADER_GATHER_COMPARISON = D3D11_FORMAT_SUPPORT_SHADER_GATHER_COMPARISON.0 as u32;
        const SHADER_LOAD = D3D11_FORMAT_SUPPORT_SHADER_LOAD.0 as u32;
        const SHADER_SAMPLE = D3D11_FORMAT_SUPPORT_SHADER_SAMPLE.0 as u32;
        const SHADER_SAMPLE_COMPARISON = D3D11_FORMAT_SUPPORT_SHADER_SAMPLE_COMPARISON.0 as u32;
        const SHADER_SAMPLE_MONO_TEXT = D3D11_FORMAT_SUPPORT_SHADER_SAMPLE_MONO_TEXT.0 as u32;
        const SO_BUFFER = D3D11_FORMAT_SUPPORT_SO_BUFFER.0 as u32;
        const TEXTURE1D = D3D11_FORMAT_SUPPORT_TEXTURE1D.0 as u32;
        const TEXTURE2D = D3D11_FORMAT_SUPPORT_TEXTURE2D.0 as u32;
        const TEXTURE3D = D3D11_FORMAT_SUPPORT_TEXTURE3D.0 as u32;
        const TEXTURECUBE = D3D11_FORMAT_SUPPORT_TEXTURECUBE.0 as u32;
        const TYPED_UNORDERED_ACCESS_VIEW = D3D11_FORMAT_SUPPORT_TYPED_UNORDERED_ACCESS_VIEW.0 as u32;
        const VIDEO_ENCODER = D3D11_FORMAT_SUPPORT_VIDEO_ENCODER.0 as u32;
        const VIDEO_PROCESSOR_INPUT = D3D11_FORMAT_SUPPORT_VIDEO_PROCESSOR_INPUT.0 as u32;
        const VIDEO_PROCESSOR_OUTPUT = D3D11_FORMAT_SUPPORT_VIDEO_PROCESSOR_OUTPUT.0 as u32;
    }
}
