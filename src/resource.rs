use bitflags::bitflags;
use d3d11_ffi::Direct3D11::*;

pub use d3d11_ffi::Direct3D11::D3D11_SUBRESOURCE_DATA;

#[repr(i32)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usage {
    #[default]
    Default = D3D11_USAGE_DEFAULT.0,
    Immutable = D3D11_USAGE_IMMUTABLE.0,
    Dynamic = D3D11_USAGE_DYNAMIC.0,
    Staging = D3D11_USAGE_STAGING.0,
}

impl Usage {
    pub fn is_immutable(&self) -> bool {
        *self == Self::Immutable
    }
}

bitflags! {
    #[derive(Clone, Debug)]
    pub struct BindFlags: i32 {
        const VERTEX_BUFFER = D3D11_BIND_VERTEX_BUFFER.0;
        const INDEX_BUFFER = D3D11_BIND_INDEX_BUFFER.0;
        const CONSTANT_BUFFER = D3D11_BIND_CONSTANT_BUFFER.0;
        const SHADER_RESOURCE = D3D11_BIND_SHADER_RESOURCE.0;
        const STREAM_OUTPUT = D3D11_BIND_STREAM_OUTPUT.0;
        const RENDER_TARGET = D3D11_BIND_RENDER_TARGET.0;
        const DEPTH_STENCIL = D3D11_BIND_DEPTH_STENCIL.0;
        const UNORDERED_ACCESS = D3D11_BIND_UNORDERED_ACCESS.0;
        const DECODER = D3D11_BIND_DECODER.0;
        const VIDEO_ENCODER = D3D11_BIND_VIDEO_ENCODER.0;
    }
}

bitflags! {
    #[derive(Clone, Debug, Default)]
    pub struct CpuAccessFlags: i32 {
        const WRITE = D3D11_CPU_ACCESS_WRITE.0;
        const READ = D3D11_CPU_ACCESS_READ.0;
    }
}

bitflags! {
    #[derive(Clone, Debug, Default)]
    pub struct ResourceMiscFlags: i32 {
        const GENERATE_MIPS = D3D11_RESOURCE_MISC_GENERATE_MIPS.0;
        const SHARED = D3D11_RESOURCE_MISC_SHARED.0;
        const TEXTURECUBE = D3D11_RESOURCE_MISC_TEXTURECUBE.0;
        const DRAWINDIRECT_ARGS = D3D11_RESOURCE_MISC_DRAWINDIRECT_ARGS.0;
        const BUFFER_ALLOW_RAW_VIEWS = D3D11_RESOURCE_MISC_BUFFER_ALLOW_RAW_VIEWS.0;
        const BUFFER_STRUCTURED = D3D11_RESOURCE_MISC_BUFFER_STRUCTURED.0;
        const RESOURCE_CLAMP = D3D11_RESOURCE_MISC_RESOURCE_CLAMP.0;
        const SHARED_KEYEDMUTEX = D3D11_RESOURCE_MISC_SHARED_KEYEDMUTEX.0;
        const GDI_COMPATIBLE = D3D11_RESOURCE_MISC_GDI_COMPATIBLE.0;
        const SHARED_NTHANDLE = D3D11_RESOURCE_MISC_SHARED_NTHANDLE.0;
        const RESTRICTED_CONTENT = D3D11_RESOURCE_MISC_RESTRICTED_CONTENT.0;
        const RESTRICT_SHARED_RESOURCE = D3D11_RESOURCE_MISC_RESTRICT_SHARED_RESOURCE.0;
        const RESTRICT_SHARED_RESOURCE_DRIVER = D3D11_RESOURCE_MISC_RESTRICT_SHARED_RESOURCE_DRIVER.0;
        const GUARDED = D3D11_RESOURCE_MISC_GUARDED.0;
        const TILE_POOL = D3D11_RESOURCE_MISC_TILE_POOL.0;
        const TILED = D3D11_RESOURCE_MISC_TILED.0;
        const HW_PROTECTED = D3D11_RESOURCE_MISC_HW_PROTECTED.0;
    }
}

pub trait Resource: Sized {
    fn to_ffi_resource(&self) -> ID3D11Resource;
    fn from_ffi_resource(resource: ID3D11Resource) -> Option<Self>;

    fn get_device(&self) -> crate::Device {
        crate::Device(unsafe { self.to_ffi_resource().GetDevice() }.unwrap())
    }

    unsafe fn get_context(&self) -> crate::DeviceContext {
        self.get_device().get_immediate_context()
    }
}

#[macro_export]
macro_rules! impl_resource {
    ($name:ident) => {
        impl_device_child!($name);
        impl $crate::resource::Resource for $name {
            fn to_ffi_resource(&self) -> ID3D11Resource {
                self.0.clone().into()
            }

            fn from_ffi_resource(resource: ID3D11Resource) -> Option<Self> {
                use d3d11_ffi::core::Interface;
                resource.cast().ok().map(Self)
            }
        }
    };
}
