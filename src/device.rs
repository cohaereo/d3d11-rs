use windows::Win32::{
    Foundation::HINSTANCE,
    Graphics::{Direct3D::*, Direct3D11::*, Dxgi::IDXGIAdapter},
};

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
    util::{to_pcstr, wrap_option_out_result},
    Buffer, BufferDesc, DepthStencilView, DepthStencilViewDesc, Query, Resource,
};

// TODO(cohae): How do we handle newer versions of the API? eg. ID3D11Device1
#[repr(transparent)]
pub struct Device(pub(crate) ID3D11Device);

// Initialization
impl Device {
    /// Temporary method for creating a device
    /// ImmediateContext can be retrieved by calling `device.immediate_context()`
    // TODO:  Abstract away IDXGIAdapter -> dxgi::Adapter
    pub fn create(adapter: Option<IDXGIAdapter>) -> crate::Result<Self> {
        let mut device = None;
        let driver_type = if adapter.is_some() {
            D3D_DRIVER_TYPE_UNKNOWN
        } else {
            D3D_DRIVER_TYPE_HARDWARE
        };

        unsafe {
            D3D11CreateDevice(
                adapter.map(|a| a.into()).as_ref(),
                driver_type,
                HINSTANCE::default(),
                // Default::default(),
                D3D11_CREATE_DEVICE_DEBUG,
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

// Methods
impl Device {
    pub fn immediate_context(&self) -> DeviceContext {
        DeviceContext(
            unsafe { self.0.GetImmediateContext() }.expect("Failed to get immediate context"),
        )
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
        if let Some(ref id) = initial_data {
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
        if let Some(ref id) = initial_data {
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
        if let Some(ref id) = initial_data {
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
        if let Some(ref desc) = desc {
            validate_input!(!desc.format.is_typeless(), "Format must not be typeless");
        }

        let inner = wrap_option_out_result(|out| unsafe {
            self.0.CreateRenderTargetView(
                &resource.to_ffi_resource(),
                desc.map(|d| d.as_ffi()),
                out,
            )
        })?;

        Ok(RenderTargetView(inner))
    }

    pub fn create_depth_stencil_view(
        &self,
        resource: &impl Resource,
        desc: Option<&DepthStencilViewDesc>,
    ) -> crate::Result<DepthStencilView> {
        let inner = wrap_option_out_result(|out| unsafe {
            self.0.CreateDepthStencilView(
                &resource.to_ffi_resource(),
                desc.map(|d| d.as_ffi()),
                out,
            )
        })?;

        Ok(DepthStencilView(inner))
    }

    pub fn create_shader_resource_view(
        &self,
        resource: &impl Resource,
        desc: Option<&ShaderResourceViewDesc>,
    ) -> crate::Result<ShaderResourceView> {
        let inner = wrap_option_out_result(|out| unsafe {
            self.0.CreateShaderResourceView(
                &resource.to_ffi_resource(),
                desc.map(|d| d.as_ffi()),
                out,
            )
        })?;

        Ok(ShaderResourceView(inner))
    }

    pub fn create_input_layout(
        &self,
        desc: &[InputElementDesc],
        shader_bytecode: &[u8],
    ) -> crate::Result<InputLayout> {
        let mut strings = vec![];
        let mut descs_ffi = vec![];
        for d in desc {
            let (cstring, pstring) = to_pcstr(d.semantic_name);
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
}
